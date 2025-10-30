#[cfg(test)]
mod test {
    use crate::{LuaType, VirtualWorkspace};

    #[test]
    fn optional_chaining_simple_field() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def(
            r#"
            obj = {{ b = { b = 1}}, 2, 3}
            a = obj[1]?.b?.b
            "#,
        );

        // Erwartet: Integer | nil
        let expected = LuaType::IntegerConst(1);
        let a_ty = ws.expr_ty("a");
        assert_eq!(a_ty, expected);
    }

}