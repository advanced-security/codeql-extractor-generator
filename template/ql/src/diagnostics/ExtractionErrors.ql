/**
 * @name Extraction errors
 * @description List all extraction errors for files in the source code directory.
 * @kind diagnostic
 * @id {{ config.name }}/diagnostics/extraction-errors
 */

import {{ config.name }}
import codeql.{{ config.name }}.Diagnostics

/** Gets the SARIF severity to associate an error. */
int getSeverity() { result = 2 }

from ExtractionError error, File f
where
  f = error.getLocation().getFile() and
  exists(f.getRelativePath())
select error, "Extraction failed in " + f + " with error " + error.getMessage(), getSeverity()
