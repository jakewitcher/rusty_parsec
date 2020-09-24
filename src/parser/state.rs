use super::result::Position;

#[derive(Clone, Debug, PartialEq)]
enum LineStart {
    FirstLine,
    Index(usize),
}

/// ```ParserState``` is used to track the state of the parser. It maintains a reference to the string value being parsed and the current position of the parser as well as a history of all previous positions. 
/// ```ParserState``` also includes functionality for moving the current position of the parser forward and backward as well as tracking line and column numbers.
pub struct ParserState {
    input: String,
    current_slice_start: usize,
    prev_slice_start: Vec<usize>,
    current_line_start: LineStart,
    prev_line_start: Vec<LineStart>,
    marker: Option<usize>,
}

impl ParserState {
    /// ```new``` creates a new instance of the ```ParserState``` struct.
    pub(in crate::parser) fn new(input: String) -> ParserState {

        ParserState {
            input,
            current_slice_start: 0,
            prev_slice_start: vec![0],
            current_line_start: LineStart::FirstLine,
            prev_line_start: vec![],
            marker: None,
        }
    }

    /// ```len``` returns the length of the input being parsed.
    pub(in crate::parser) fn len(&self) -> usize {
        self.input.len()
    }

    /// ```get_remaining_input``` returns a slice of the input from the current position of the parser to the end of the input string. 
    /// ```get_remaining_input``` panics if the current position of the parser has exceeded the length of the input.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let mut state = ParserState::new("hello, world".to_string());
    ///
    /// state.move_state_forward("hello".len());
    /// let remaining_input = state.get_remaining_input();
    ///
    /// assert_eq!(", world", remaining_input);
    /// ```
    pub(in crate::parser) fn get_remaining_input(&self) -> &str {
        if self.current_slice_start > self.len() {
            panic!(
                format!("starting slice at {} will exceed the input length of {}",
                self.current_slice_start,
                self.len())
            )
        }

        &self.input[self.current_slice_start..]
    }

    /// ```move_state_forward``` moves the current position of the parser forward by the number of indicies specified with the ```increment``` parameter.
    /// When the position of the parser is moved, the characters between the current parser position and the new parser position are checked for newlines so that 
    /// the line number is tracked as well.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let mut state = ParserState::new("hello\nworld".to_string());
    ///        
    /// state.move_state_forward("hello\nwo".len());
    ///
    /// let position = state.get_position();
    /// 
    /// let row = 2;
    /// let column = 3;
    /// let index = 8;
    /// 
    /// let expected = Position::new(row, column, index);
    ///
    /// assert_eq!(expected, position);
    /// ```
    pub(in crate::parser) fn move_state_forward(&mut self, increment: usize) {
        if self.current_slice_start + increment > self.len() {
            panic!(
                format!("incrementing starting index {} by {} will exceed the input length of {}",
                self.current_slice_start, 
                increment, 
                self.len())
            );
        }

        self.move_newlines_forward(increment);
        self.move_slice_start_forward(increment);
    }

    fn move_slice_start_forward(&mut self, increment: usize) {
        self.prev_slice_start.push(self.current_slice_start);
        self.current_slice_start += increment;
    }

    fn move_newlines_forward(&mut self, increment: usize) {
        let current_slice = 
            self.get_slice(increment).unwrap_or_default();

        let chars = current_slice.chars();
        let mut char_index = 0;

        for c in chars {
            if c == '\n' {
                self.prev_line_start.push(self.current_line_start.clone());
                
                self.current_line_start = LineStart::Index(self.current_slice_start + char_index);
            }
            
            char_index += c.len_utf8();
        }
    }

    fn move_state_back(&mut self) {
        self.move_slice_start_back();
        self.move_newlines_back();
    }

    fn move_slice_start_back(&mut self) {
        match self.prev_slice_start.pop() {
            Some(0) if self.prev_slice_start.is_empty() => {
                self.prev_slice_start = vec![0];
                self.current_slice_start = 0;
            },
            Some(i) => {
                self.current_slice_start = i;
            },
            None => {
                panic!("slice start index cannot be moved back, vector of index history is empty")
            },
        }
    }

    fn move_newlines_back(&mut self) {
        while ParserState::line_start_is_greater_than_slice_start(self.prev_line_start.last(), self.current_slice_start) {
            self.prev_line_start.pop();
        }

        self.current_line_start = 
            self.prev_line_start.pop()
                .unwrap_or(LineStart::FirstLine);
    }

    fn line_start_is_greater_than_slice_start(line_start: Option<&LineStart>, slice_start: usize) -> bool {
        match line_start {
            Some(LineStart::Index(i)) => *i > slice_start,
            _ => false,
        }
    }

    /// ```mark``` sets a marker for the current position of the parser. This marker is used by parsers that allow for the state to be reverted to
    /// an earlier position if a fatal error occurs.
    pub(in crate::parser) fn mark(&mut self) {
        self.marker = Some(self.current_slice_start);
    }

    /// ```revert``` uses the marker set by ```mark``` to move the position of the parser to a previous state.
    pub(in crate::parser) fn revert(&mut self) {
        match self.marker {
            Some(marker) => {
                while self.current_slice_start != marker {
                    self.move_state_back();
                }
                self.remove_mark();
            },
            _ => ()
        }
    }

    /// ```remove_mark``` removes any markers that have been set by ```mark```.
    pub(in crate::parser) fn remove_mark(&mut self) {
        self.marker = None;
    }

    /// ```get_slice``` attempts to get a slice of the input to be evaluated by a parser function. The starting position of the slice
    /// is determined by the current position of the parser state and the end point of the slice is determined by the caller.
    /// ```get_slice``` returns ```None``` if the slice requested exceeds the length of the input string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let mut state = ParserState::new("hello, world".to_string());
    /// state.move_state_forward("hello, ".len());
    /// 
    /// let slice = state.get_slice(4);
    /// let expected = Some("worl".to_string());
    /// 
    /// assert_eq!(expected, slice);
    /// ```
    pub(in crate::parser) fn get_slice(&self, length: usize) -> Option<String> {

        let slice_end = self.current_slice_start + length;

        if slice_end > self.len() {
            None
        } else {
            let slice = &self.input[self.current_slice_start..slice_end];
            Some(String::from(slice))
        }
    }

    /// ```get_position``` returns the current position of the parser state using the ```Position``` struct. 
    /// ```Position``` includes the line number, column number, and index of the current parser state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let mut state = ParserState::new("hello, world".to_string());
    /// state.move_state_forward("hello, ".len());
    /// 
    /// let position = state.get_position();
    /// let expected = Position::new(1, 8, 7);
    /// 
    /// assert_eq!(expected, position);
    /// ```
    pub(in crate::parser) fn get_position(&self) -> Position {
        Position::new(self.get_line_number(), self.get_column_number(), self.get_index())
    }

    fn get_index(&self) -> usize {
        self.current_slice_start
    }

    fn get_line_number(&self) -> usize {
        self.prev_line_start.len() + 1
    }

    fn get_column_number(&self) -> usize {
        match self.current_line_start {
            LineStart::FirstLine => self.current_slice_start + 1,
            LineStart::Index(index) => self.current_slice_start - index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LineStart, ParserState};

    #[test]
    fn gets_remaining_slice_of_input_to_be_parsed() {
        let mut state = ParserState::new("hello, world".to_string());
        
        state.move_state_forward("hello".len());
        let remaining_input = state.get_remaining_input();

        assert_eq!(", world", remaining_input);
    }

    #[test]
    #[should_panic(expected = "will exceed the input length")]
    fn get_remaining_slice_panics_if_slice_start_exceeds_input_length() {
        let mut state = ParserState::new("hello".to_string());
        state.current_slice_start = 7;
        state.get_remaining_input();
    }

    #[test]
    fn move_state_forward_increments_current_slice_start_by_one() {
        let mut state = ParserState::new("hello".to_string());
        
        state.move_state_forward('h'.len_utf8());

        assert_eq!('h'.len_utf8(), state.current_slice_start);
    }

    #[test]
    fn move_state_forward_increments_current_slice_start_by_many() {
        let mut state = ParserState::new("hello, world".to_string());
        
        state.move_state_forward("hello".len());

        assert_eq!("hello".len(), state.current_slice_start);
    }

    #[test]
    fn move_state_forward_increments_current_line_start() {
        let mut state = ParserState::new("hello\nworld".to_string());
        
        state.move_state_forward("hello\nwo".len());

        let expected = LineStart::Index(5);

        assert_eq!(expected, state.current_line_start);
    }

    #[test]
    fn move_state_forward_does_not_increment_current_line_start() {
        let mut state = ParserState::new("hello\nworld".to_string());
        
        state.move_state_forward("hello".len());

        let expected = LineStart::FirstLine;

        assert_eq!(expected, state.current_line_start);
    }

    #[test]
    #[should_panic(expected = "will exceed the input length")]
    fn move_state_forward_panics_if_increment_exceeds_input_length() {
        let mut state = ParserState::new("hello".to_string());

        state.move_state_forward(7);
    }

    #[test]
    fn move_state_back_sets_current_slice_start_back_one() {
        let mut state = ParserState::new("hello, world".to_string());

        state.move_state_forward("hello".len());
        state.move_state_forward(", ".len());

        assert_eq!(7, state.current_slice_start);

        state.move_state_back();

        assert_eq!(5, state.current_slice_start);

        state.move_state_back();

        assert_eq!(0, state.current_slice_start);
    }

    #[test]
    fn move_state_back_sets_current_line_start_back_one() {
        let mut state = ParserState::new("hello\n, \nworld".to_string());

        state.move_state_forward("hello\n,".len());
        state.move_state_forward(" \nw".len());

        assert_eq!(LineStart::Index(8), state.current_line_start);

        state.move_state_back();

        assert_eq!(LineStart::Index(5), state.current_line_start);

        state.move_state_back();

        assert_eq!(LineStart::FirstLine, state.current_line_start);
    }

    #[test]
    fn marks_current_slice_start_and_reverts_state_back_to_marker() {
        let mut state = ParserState::new("hello, world".to_string());

        state.move_state_forward("hello".len());

        state.mark();

        state.move_state_forward(", ".len());
        state.move_state_forward("world".len());

        state.revert();

        assert_eq!(5, state.current_slice_start);
    }

    #[test]
    fn calling_revert_with_no_change_in_state_does_not_affect_parser_state() {
        let mut state = ParserState::new("hello, world".to_string());

        state.move_state_forward("hello".len());
        state.mark();
        state.revert();

        assert_eq!(5, state.current_slice_start);
    }

    #[test]
    fn calling_revert_with_no_marker_does_not_affect_parser_state() {
        let mut state = ParserState::new("hello, world".to_string());
        assert_eq!(0, state.current_slice_start);

        state.revert();

        assert_eq!(0, state.current_slice_start);
    }
}
