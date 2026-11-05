/**
 * Provides {{ config.name }}-specific definitions for use in the taint tracking library.
 */

private import codeql.Locations
private import codeql.dataflow.TaintTracking
private import DataFlowImplSpecific

module {{ config.display_name }}TaintTracking implements InputSig<Location, {{ config.display_name }}DataFlow> {
  import TaintTrackingPrivate
}
