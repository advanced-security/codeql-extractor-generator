/**
 * Provides the module `TaintTracking`.
 */

private import {{ config.name }}

/**
 * Provides a library for performing local (intra-procedural) and global
 * (inter-procedural) taint-tracking analyses.
 */
module TaintTracking {
  import codeql.{{ config.name }}.dataflow.internal.TaintTrackingImpl::Public
  private import codeql.{{ config.name }}.dataflow.internal.DataFlowImplSpecific
  private import codeql.{{ config.name }}.dataflow.internal.TaintTrackingImplSpecific
  private import codeql.dataflow.TaintTracking
  private import {{ config.name }}
  import TaintFlowMake<Location, {{ config.display_name }}DataFlow, {{ config.display_name }}TaintTracking>
}