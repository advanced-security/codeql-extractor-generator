/**
 * Internal: Control flow graph construction for {{ config.display_name }}.
 */

private import {{ config.name }}
// CFG
import codeql.controlflow.Cfg
import Completion
private import Scope as Scope
private import codeql.{{ config.name }}.controlflow.ControlFlowGraph as Cfg

// private import codeql.Locations
private module Implementation implements InputSig<Location> {
  private import {{ config.name }} as {{ config.display_name }}
  private import Completion as C
  private import Splitting as S

  class AstNode = {{ config.display_name }}::AstNode;

  class Completion = C::Completion;

  predicate completionIsNormal = C::completionIsNormal/1;

  predicate completionIsSimple = C::completionIsSimple/1;

  predicate completionIsValidFor = C::completionIsValidFor/2;

  /** An AST node with an associated control-flow graph. */
  class CfgScope = Scope::CfgScope;

  class SplitKindBase = S::TSplitKind;

  class Split = S::Split;

  class SuccessorType = Cfg::SuccessorType;

  CfgScope getCfgScope(AstNode e) {
    exists(AstNode p | p = e.getParent() |
      result = p
      or
      not p instanceof CfgScope and result = getCfgScope(p)
    )
  }

  int maxSplits() { result = 0 }

  /** Holds if `first` is first executed when entering `scope`. */
  predicate scopeFirst(CfgScope scope, AstNode first) { scope.scopeFirst(first) }

 /** Holds if `scope` is exited when `last` finishes with completion `c`. */
  predicate scopeLast(CfgScope scope, AstNode last, Completion c) { scope.scopeLast(last, c) }

  predicate successorTypeIsSimple(SuccessorType t) { t instanceof Cfg::NormalSuccessor }

  predicate successorTypeIsCondition(SuccessorType t) { t instanceof Cfg::BooleanSuccessor }

  SuccessorType getAMatchingSuccessorType(Completion c) { result = c.getAMatchingSuccessorType() }

  predicate isAbnormalExitType(SuccessorType t) { none() }
}

private module CfgImpl = Make<Location, Implementation>;

import CfgImpl

// TODO: CFG construction for {{ config.display_name }}.

/**
 * A literal value in a {{ config.display_name }} program.
 */
class LiteralTree extends LeafTree instanceof Literals { }

{%- for literal in ts.literals %}
/**
 * A {{ literal }} literal value in a {{ config.display_name }} program.
 */
class {{ literal }}LiteralTree extends LeafTree instanceof {{ literal }}Literal { }
{%- endfor %}

{%- if ts.loops.size != 0  %}
/**
 * A loop expression in a {{ config.display_name }} program.
 */
abstract class LoopingExprTree extends PostOrderTree {
  override predicate propagatesAbnormal(AstNode child) { child = this.getLoopBody() }

  abstract {{ ts.block_node }} getLoopBody();

  /**
   * Gets the node to execute when continuing the loop; either after
   * executing the last node in the body or after an explicit `continue`.
   */
  abstract AstNode getLoopContinue();

  override predicate succ(AstNode pred, AstNode succ, Completion c) {
    // Edge back to the start for final expression and continue expressions
    last(this.getLoopBody(), pred, c) and
    completionIsNormal(c) and
    first(this.getLoopContinue(), succ)
  }
}

{% for loop in ts.loops %}
/**
 * A {{ loop }} in a {{ config.display_name }} program.
 */
class {{ loop }}Tree extends LoopingExprTree instanceof {{ loop }} {
  override {{ ts.block_node }} getLoopBody() { result = {{ loop }}.super.getBody() }

  override AstNode getLoopContinue() { result = {{ loop }}.super.getCondition() }

  override predicate first(AstNode node) { first({{ loop }}.super.getBody(), node) }
}
{% endfor %}
{%- endif %}
