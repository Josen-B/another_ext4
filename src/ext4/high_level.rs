//! High-level operations of Ext4 filesystem.
//!
//! This module provides path-based operations. An object can be
//! located in the filesystem by its relative or absolute path.
//!
//! Some operations such as `read`, `write`, `setattr` do not involve
//! file location. They are implemented in the `low_level` module.
//! High-level and low-level operations can be used together to
//! implement more complex operations.

use super::Ext4;
use crate::ext4_defs::*;
use crate::prelude::*;
use crate::return_error;

impl Ext4 {
    /// Look up an object in the filesystem recursively.
    ///
    /// # Params
    ///
    /// * `root` - The inode id of the root directory for search.
    /// * `path` - The relative path of the object to be opened.
    ///
    /// # Return
    ///
    /// `Ok(inode)` - Inode id of the object
    ///
    /// # Error
    ///
    /// * `ENOTDIR` - Any parent along `path` is not a directory.
    /// * `ENOENT` - The object does not exist.
    pub fn generic_lookup(&self, root: InodeId, path: &str) -> Result<InodeId> {
        trace!("generic_lookup({}, {})", root, path);
        // Search from the given parent inode
        let mut cur = root;
        let search_path = Self::split_path(path);
        // Search recursively
        for path in search_path.iter() {
            cur = self.lookup(cur, path)?;
        }
        Ok(cur)
    }

    /// (DEPRECATED) Open a file in the filesystem.
    ///
    /// # Params
    ///
    /// * `root` - The inode id of the root directory for search.
    /// * `path` - The path of the object to be opened.
    /// * `flags` - The open flags. Creation (O_CREAT, O_EXCL, O_NOCTTY) flags
    ///             will be ignored.
    ///
    /// # Return
    ///
    /// `Ok(fh)` - File handler
    /// 
    /// # Error
    /// 
    /// * `ENOENT` - The file does not exist.
    /// * `EISDIR` - The file is a directory.
    #[deprecated]
    pub fn generic_open(&self, root: InodeId, path: &str, flags: OpenFlags) -> Result<FileHandler> {
        let inode_id = self.generic_lookup(root, path)?;
        let inode = self.read_inode(inode_id);
        // Check file type
        if !inode.inode.is_file() {
            return_error!(ErrCode::EISDIR, "File {} is not a regular file", path);
        }
        Ok(FileHandler::new(inode.id, flags, inode.inode.size()))
    }

    /// Create an object in the filesystem.
    ///
    /// This function will perform recursive-creation i.e. if the parent
    /// directory does not exist, it will be created as well.
    ///
    /// # Params
    ///
    /// * `root` - The inode id of the starting directory for search.
    /// * `path` - The relative path of the object to create.
    /// * `mode` - file mode and type to create
    ///
    /// # Return
    ///
    /// `Ok(inode)` - Inode id of the created object
    ///
    /// # Error
    ///
    /// * `ENOTDIR` - Any parent along `path` is not a directory.
    /// * `EEXIST` - The object already exists.
    pub fn generic_create(&self, root: InodeId, path: &str, mode: InodeMode) -> Result<InodeId> {
        // Search from the given parent inode
        let mut cur = self.read_inode(root);
        let search_path = Self::split_path(path);
        // Search recursively
        for (i, path) in search_path.iter().enumerate() {
            if !cur.inode.is_dir() {
                return_error!(ErrCode::ENOTDIR, "Parent {} is not a directory", cur.id);
            }
            match self.dir_find_entry(&cur, &path) {
                Ok(id) => {
                    if i == search_path.len() - 1 {
                        // Reach the object and it already exists
                        return_error!(ErrCode::EEXIST, "Object {}/{} already exists", root, path);
                    }
                    cur = self.read_inode(id);
                }
                Err(e) => {
                    if e.code() != ErrCode::ENOENT {
                        return_error!(e.code(), "Unexpected error: {:?}", e);
                    }
                    let mut child = if i == search_path.len() - 1 {
                        // Reach the object, create it
                        self.create_inode(mode)?
                    } else {
                        // Create parent directory
                        self.create_inode(InodeMode::DIRECTORY | InodeMode::ALL_RWX)?
                    };
                    self.link_inode(&mut cur, &mut child, path)?;
                    cur = child;
                }
            }
        }
        Ok(cur.id)
    }

    /// Remove an object from the filesystem.
    ///
    /// # Params
    ///
    /// * `root` - The inode id of the starting directory for search.
    /// * `path` - The path of the object to remove.
    ///
    /// # Error
    ///
    /// * `ENOENT` - The object does not exist.
    /// * `ENOTEMPTY` - The object is a non-empty directory.
    pub fn generic_remove(&self, root: InodeId, path: &str) -> Result<()> {
        // Get the parent directory path and the file name
        let mut search_path = Self::split_path(path);
        let file_name = &search_path.split_off(search_path.len() - 1)[0];
        let parent_path = search_path.join("/");
        // Get the parent directory inode
        let parent_id = self.generic_lookup(root, &parent_path)?;
        // Get the child inode
        let child_id = self.generic_lookup(parent_id, &file_name)?;
        let mut parent = self.read_inode(parent_id);
        let mut child = self.read_inode(child_id);
        // Check if child is a non-empty directory
        if child.inode.is_dir() && self.dir_list_entries(&child).len() > 2 {
            return_error!(ErrCode::ENOTEMPTY, "Directory {} not empty", path);
        }
        // Unlink the file
        self.unlink_inode(&mut parent, &mut child, file_name, true)
    }

    /// A helper function to split a path by '/'
    fn split_path(path: &str) -> Vec<String> {
        let path = path.trim_start_matches("/");
        if path.is_empty() {
            return vec![]; // root
        }
        path.split("/").map(|s| s.to_string()).collect()
    }
}
