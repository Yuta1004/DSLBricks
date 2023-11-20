use compiler::designer::constraint::ctime::declare_constraints;
use compiler::designer::design::DSLGeneratable;

// 1. ユーザに宣言を要請する制約
declare_constraints! {
    // StaticValue : プログラム上に直接記述される値
    new StaticValue;

    // Calculatable : 計算可能
    new Calculatable;
}

// 2. 自動導出可能な制約
declare_constraints! {
    // RValue : 右辺値として評価可能
    compose RValue <= StaticValue;
}
