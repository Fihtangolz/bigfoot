
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
