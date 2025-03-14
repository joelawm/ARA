/*-------------
/file.rs

This file is for the handling of errors in the application.
-------------*/
#[derive(Debug)]
pub enum Error {
	NewSettingsError(String),
}