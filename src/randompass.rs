use random_string::generate;

pub fn random() -> String {
    let charset = "abcdefghijklmnopqrtstuvwxyzABCDEFGHIJLMNOPQRSTUVWXYZ1234567890!@#$%&*";
    generate(15, charset)
}