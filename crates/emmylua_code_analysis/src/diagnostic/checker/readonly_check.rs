use emmylua_parser::{LuaAssignStat, LuaAst, LuaAstNode, LuaVarExpr};
use rowan::{NodeOrToken, TextRange};

use crate::{
    DiagnosticCode, LuaSemanticDeclId, PropertyDeclFeature, SemanticDeclLevel, SemanticModel,
};

use super::{Checker, DiagnosticContext};

pub struct ReadOnlyChecker;

impl Checker for ReadOnlyChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::ReadOnly];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let root = semantic_model.get_root().clone();
        for ast_node in root.descendants::<LuaAst>() {
            match ast_node {
                LuaAst::LuaAssignStat(assign_stat) => {
                    check_assign_stat(context, semantic_model, &assign_stat);
                }
                // need check?
                LuaAst::LuaFuncStat(_) => {}
                // we need known function is readonly
                LuaAst::LuaCallExpr(_) => {}
                _ => {}
            }
        }
    }
}

fn check_and_report_semantic_id(
    context: &mut DiagnosticContext,
    range: TextRange,
    semantic_decl_id: LuaSemanticDeclId,
) -> Option<()> {
    // TODO filter self
    let property_index = context.db.get_property_index();
    if let Some(property) = property_index.get_property(&semantic_decl_id) {
        if property
            .decl_features
            .has_feature(PropertyDeclFeature::ReadOnly)
        {
            context.add_diagnostic(
                DiagnosticCode::ReadOnly,
                range,
                t!("The variable is marked as readonly and cannot be assigned to.").to_string(),
                None,
            );
        }
    }

    Some(())
}

fn check_assign_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    assign_stat: &LuaAssignStat,
) -> Option<()> {
    let (vars, _) = assign_stat.get_var_and_expr_list();
    for var in vars {
        let node_or_token = NodeOrToken::Node(var.syntax().clone());
        let semantic_decl_id =
            semantic_model.find_decl(node_or_token, SemanticDeclLevel::default());
        if let Some(semantic_decl_id) = semantic_decl_id {
            check_and_report_semantic_id(context, var.get_range(), semantic_decl_id);
        }

        match var {
            LuaVarExpr::IndexExpr(index_expr) => {
                let prefix_node = index_expr.get_prefix_expr()?;
                let node_or_token = NodeOrToken::Node(prefix_node.syntax().clone());
                let semantic_decl_id =
                    semantic_model.find_decl(node_or_token, SemanticDeclLevel::default());
                if let Some(semantic_decl_id) = semantic_decl_id {
                    check_and_report_semantic_id(
                        context,
                        prefix_node.get_range(),
                        semantic_decl_id,
                    );
                }
            }
            _ => {}
        }
    }

    Some(())
}
