/**
 * @name Successfully extracted files
 * @description Lists all files in the source code directory that were extracted
 *   without encountering an error in the file.
 * @kind diagnostic
 * @id {{ config.name }}/diagnostics/successfully-extracted-files
 * @tags successfully-extracted-files
 */

import {{ config.name }}
import codeql.{{ config.name }}.Diagnostics


from File f
where
  not exists(ExtractionError e | e.getLocation().getFile() = f) and
  exists(f.getRelativePath())
select f, ""
