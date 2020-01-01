
pub enum SQLStmt {
    ControlStmt(ControlStmt)
}

pub enum ControlStmt {
    CallStmt(RoutineInvocation),
    ReturnStmt(ReturnStmt)
}

pub struct RoutineInvocation {

}

struct NULL;

pub enum ReturnStmt {
    NULL,
    ValueExpr(ValueExpr)
}

pub enum ValueExpr {

}

pub enum DefaultClause {
    User,
    CurrentUser,
    CurrentRole,
    SessionUser,
    SystemUser,
    CurrentCatalog,
    CurrentSchema,
    CurrentPath,
}

pub enum TableScope {
    Global,
    Local,
}

pub enum TableCommitAction {
    Preserve,
    Delete,
}

pub struct TableElem {
    
}

pub struct TypedClause {

}

pub struct SubqueryClause {

}

pub enum TableContentsSource {
    ElemList(vec<TableElem>),
    TypedClause(TypedClause),
    SubqueryClause(SubqueryClause), 
}

pub struct TableDefinition {
    Scope: Option<TableScope>,
    Name: str,
    Action: Option<TableCommitAction>,
    IsSysVersioning: bool,
    Source: TableContentsSource,
}

