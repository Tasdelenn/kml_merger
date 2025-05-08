# KML Merger & Stylizer

KML Merger & Stylizer is a web application that allows users to upload multiple KML files, customize their styles (line color, line width, fill color, fill opacity), and merge them into a single downloadable KML file.

## Features

*   Upload multiple KML files via selection or drag-and-drop.
*   Customize style for each KML file individually:
    *   Line Color
    *   Line Width
    *   Fill Color
    *   Fill Opacity
*   Merge selected KML files into one.
*   Specify an optional output filename.
*   Dark/Light mode for user interface.
*   Responsive design.

## Project Structure

```
.gitignore
Cargo.lock
Cargo.toml
package.json
README.md
src/
  error.rs         # Custom error types
  kml_builder.rs   # Builds the final KML string
  kml_merger.rs    # Merges KML file data
  kml_parser.rs    # Parses KML file content
  kml_types.rs     # Defines KML-related data structures
  main.rs          # Main entry point for the Actix web server
  traits.rs        # Defines traits for KML operations
  web.rs           # Actix web handlers and routes
static/
  index.html       # Frontend HTML, CSS (Tailwind), and JavaScript
test_data/
  184-ada-10-parsel.kml # Sample KML file
  184-ada-2-parsel.kml  # Sample KML file
  184-ada-3-parsel.kml  # Sample KML file
  184-ada-6-parsel.kml  # Sample KML file
  184-ada-8-parsel.kml  # Sample KML file
  184-ada-9-parsel.kml  # Sample KML file
  merged.kml            # Example of a merged KML file
```

## Tech Stack

*   **Backend:** Rust, Actix Web
*   **Frontend:** HTML, Tailwind CSS, Vanilla JavaScript
*   **KML Parsing:** quick-xml
*   **Build Tool (Frontend):** Vite (though not strictly used for this simple static setup, `package.json` is present for potential future enhancements)

## Setup and Usage

### Prerequisites

*   Rust (latest stable version recommended)
*   Node.js and npm (for potential frontend development, not strictly required to run the compiled backend)

### Running the Application

1.  **Clone the repository (if applicable):**
    ```bash
    git clone <repository-url>
    cd kml_merger
    ```

2.  **Build and run the Rust backend:**
    ```bash
    cargo build --release
    ./target/release/kml_merger
    ```
    Alternatively, for development:
    ```bash
    cargo run
    ```
    The server will start, typically at `http://127.0.0.1:8080`.

3.  **Access the application:**
    Open your web browser and navigate to `http://127.0.0.1:8080`.

### How to Use

1.  **Select KML Files:** Click the "Select KML Files" button or drag and drop your KML files onto the designated area.
2.  **Customize Styles:** For each uploaded file, you can adjust:
    *   Line Color
    *   Line Width
    *   Fill Color
    *   Fill Opacity (%)
3.  **Set Output Filename (Optional):** Enter a desired name for the merged file (without the `.kml` extension). If left blank, it defaults to `merged.kml`.
4.  **Merge Files:** Click the "Merge Files" button.
5.  **Download:** The merged KML file will be automatically downloaded by your browser.

## Notes

*   The application expects valid KML files. Parsing errors might occur with malformed files.
*   The styling options are applied to all placemarks within a given KML file.
*   The `test_data` directory contains sample KML files that can be used for testing.
*   The `static/index.html` file contains all the frontend logic. It uses Tailwind CSS for styling, loaded via CDN.

## Future Enhancements (Potential)

*   More advanced styling options (e.g., icons, labels).
*   Individual placemark styling.
*   Preview of KML files on a map before merging.
*   Integration with a proper frontend framework if complexity increases.