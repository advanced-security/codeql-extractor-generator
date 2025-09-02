/**
 * Provides {{ config.name }}-specific definitions for use in the data flow library.
 */

 private import {{ config.name }}
 private import codeql.dataflow.DataFlow
 
 module Private {
   import DataFlowPrivate
   import DataFlowDispatch
 }
 
 module Public {
   import DataFlowPublic
 }
 
 module {{ config.display_name }}DataFlow implements InputSig<Location> {
   import Private
   import Public
 
   class ParameterNode = Private::ParameterNodeImpl;
 
   Node exprNode(DataFlowExpr e) { result = Public::exprNode(e) }
 }