# File Organizer

A simple and efficient Rust program that organizes files in a given directory into categorized subfolders based on their file types. This tool helps you quickly sort and manage your files by grouping them into folders such as images, documents, music, videos, and others.

---

## Features

- **Automatic file categorization** by file extension:
  - **Images:** jpg, jpeg, png, gif, bmp, webp, tiff, tif
  - **Documents:** txt, pdf, doc, docx, xls, xlsx, ppt, pptx, md, odt, rtf
  - **Music:** mp3, wav, ogg, flac, aac, m4a
  - **Videos:** mp4, mkv, avi, mov, wmv, flv, mpeg
  - **Others:** Any unrecognized or unsupported file extensions

- **Organized folder structure:** Creates subfolders inside an `organized` folder in the target directory.
- **Safe file moves:** Avoids overwriting existing files by checking if the target file exists.
- **Robust move operation:** Falls back to copy and delete if `rename` (move) fails, ensuring files are moved reliably.
- **Logging support:** Uses the Rust `log` crate to provide error and info messages.


## Code Overview

- **FileType enum:** Defines categories like Image, Document, Music, Video, Unknown.

- **get_file_type_mapping:** Maps file extensions to `FileType`.

- **get_files_in_directory:** Reads all files (not directories) from the specified folder.

- **get_file_type:** Determines the file type based on extension.

- **get_folder_name:** Returns folder name string for each file type.

- **move_file_to_folder:** Moves files to target folders, creates directories if needed, and handles rename errors gracefully.

- **main:** Orchestrates loading files, categorizing, and moving them.

---
