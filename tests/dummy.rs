#[cfg(test)]
mod test {
    fn test_dummy() {
        use conflicting::conflicting;

        conflicting! {
            a => {
                "a"
            },
            b => {
                "b"
            },
            c => {
                "c"
            },
        }
    }
}
