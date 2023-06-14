pub fn add(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(add(2, 2), 4);
        // assert!(add(2, 2), 4);
        // assert_eq!()
        // assert_ne!()
        // asser  assert!(
        //
        // #[test]
        // #[should_panic]'
        // to detect panicks
        // but this can introduce errors, as any panics will cause this
        // so use this 
        // #[should_panic(expected = "less than or equal to 100")]
        //
        // error messages!
        // result.contains("Carol"),
        // "Greeting did not contain name, value was `{}`",
        //
        // #[cfg(test)]
        // mod tests {
        //     #[test]
        //     fn it_works() -> Result<(), String> {
        //         if 2 + 2 == 4 {
        //             Ok(())
        //         } else {
        //             Err(String::from("two plus two does not equal four"))
        //         }
        //     }
        // }
        // #[ignore] to ignore a test unless its pecifically requested
    }
}
