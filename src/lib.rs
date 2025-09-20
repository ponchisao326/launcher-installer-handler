use std::fs::{create_dir_all, remove_file, File};
use std::io::copy;
use std::path::Path;
use launcher_request_handler::*;
use system_shutdown::{reboot, shutdown};

pub struct InstallerOptions {
    pub zip_path: String,
    pub extract_to: String,
    pub delete_zip_after: bool,
    pub restart_after_bool: bool
}

impl InstallerOptions {
    /// Create a new instance of InstallerOptions.
    pub fn new(zip_path: String, extract_to: String, delete_zip_after: bool, restart_after_bool: bool) -> Self {
        InstallerOptions {
            zip_path,
            extract_to,
            delete_zip_after,
            restart_after_bool
        }
    }

    /// Extract the ZIP file to the specified directory.
    pub fn extract(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure the extraction directory exists
        if Downloader::folder_exists(Path::new(&self.extract_to).parent().unwrap().to_str().unwrap()) == false {
            create_dir_all(Path::new(&self.extract_to).parent().unwrap())?;
        }

        let file = File::open(&self.zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = Path::new(&self.extract_to).join(file.name());

            if (*file.name()).ends_with('/') {
                create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                copy(&mut file, &mut outfile)?;
            }
        }

        // Optionally delete the ZIP file after extraction
        if self.delete_zip_after {
            remove_file(&self.zip_path)?;
        }

        // Optionally restart the system after extraction
        if self.restart_after_bool {
            self.restart_after();
        }

        Ok(())
    }

    /// Restart the system.
    fn restart_after(&mut self){
        match reboot() {
            Ok(_) => println!("System is shutting down..."),
            Err(e) => println!("Failed to shut down the system: {}", e),
        }
    }
}
