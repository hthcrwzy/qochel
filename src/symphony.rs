use once_cell::sync::Lazy;
use std::collections::HashMap;

mod play;
pub use play::Symphony;

/// QP番号を管理する
static QP_NUMBERS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    vec![
        ("QP1", "#if"),
        ("QP2", "#elif"),
        ("QP3", "#else"),
        ("QP4", "#defined"),
        ("QP5", "#ifdef"),
        ("QP6", "#ifndef"),
        ("QP7", "#define"),
        ("QP8", "#undef"),
        ("QP9", "#include"),
        ("QP10", "#line"),
        ("QP11", "#error"),
        ("QP12", "#pragma"),
        ("QP13", "_Pragma"),
    ]
    .iter()
    .copied()
    .collect()
});

/// QEK番号を管理する
static QEK_NUMBERS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    vec![("QEK1", "asm"), ("QEK2", "fortran")]
        .iter()
        .copied()
        .collect()
});

/// Qochel番号を管理する
static QOCHEL_NUMBERS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    vec![
        ("Q1", "auto"),
        ("Q2", "break"),
        ("Q3", "case"),
        ("Q4", "char"),
        ("Q5", "const"),
        ("Q6", "continue"),
        ("Q7", "default"),
        ("Q8", "do"),
        ("Q9", "double"),
        ("Q10", "else"),
        ("Q11", "enum"),
        ("Q12", "extern"),
        ("Q13", "float"),
        ("Q14", "for"),
        ("Q15", "goto"),
        ("Q16", "if"),
        ("Q16a", "inline"),
        ("Q17", "int"),
        ("Q18", "long"),
        ("Q19", "register"),
        ("Q19a", "restrict"),
        ("Q20", "return"),
        ("Q21", "short"),
        ("Q22", "signed"),
        ("Q23", "sizeof"),
        ("Q24", "static"),
        ("Q25", "struct"),
        ("Q26", "switch"),
        ("Q27", "typeof"),
        ("Q28", "union"),
        ("Q29", "unsigned"),
        ("Q30", "void"),
        ("Q31", "volatile"),
        ("Q32", "while"),
        ("Q32a", "_Alignas"),
        ("Q32b", "_Alignof"),
        ("Q32c", "_Atomic"),
        ("Q33", "_Bool"),
        ("Q34", "_Complex"),
        ("Q34a", "_Generic"),
        ("Q35", "_Imaginary"),
        ("Q36", "_Noreturn"),
        ("Q37", "_Static_assert"),
        ("Q38", "_Thread_local"),
    ]
    .iter()
    .copied()
    .collect()
});
