/**
 * Provides classes for performing local (intra-procedural) and
 * global (inter-procedural) data flow analyses.
 */

import {{ config.name }}

module DataFlow {
  private import internal.DataFlowImplSpecific
  private import codeql.dataflow.DataFlow
  import DataFlowMake<Location, {{ config.display_name }}DataFlow>
  import internal.DataFlowImpl
}
