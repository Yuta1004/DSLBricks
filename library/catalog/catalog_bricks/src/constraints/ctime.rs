use compiler::designer::constraint::ctime::declare_constraints;

// 1. ユーザに宣言を要請する制約
declare_constraints! {
    // DeclaredObject : プログラム上に直接宣言されるオブジェクト
    new DeclaredObject;

    // Identifiable : あるスコープにおいて一意に識別可能
    new Identifiable;

    // Calculatable : 計算可能
    new Calculatable;

    // Executable : 実行可能
    new Executable;
}
