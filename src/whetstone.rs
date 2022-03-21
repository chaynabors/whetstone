// auto-generated: "lalrpop 0.19.7"
// sha3: 25d0cbf2e8cc3b234fada6f6791931ad6a3ec2538837f872697d4ba9193df4
use std::collections::HashMap;
use std::str::FromStr;
use crate::ast::Variable;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Script {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::ast::Variable;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Variable),
        Variant2(bool),
        Variant3(()),
        Variant4(f64),
        Variant5(i64),
        Variant6(alloc::vec::Vec<()>),
        Variant7(String),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 16, 0, 0, 17, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 16, 0, 0, 17, 0,
        // State 2
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 3
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 4
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 5
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 6
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 7
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 8
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 9
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 10
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 17, 32,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, -20, 0, 0, -20, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, 0, -26, 0, 0, -26, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, -4, 0, 0, -4, 0,
        // State 16
        0, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, 0, -27, 0, 0, -27, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -14, -14, -14, -14, -14, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 7, 8, 0, -2, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, -7, 10, -7, -7, 11, -7, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -16, -16, -16, -16, -16, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -15, -15, -15, -15, -15, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -17, -17, -17, -17, -17, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -10, -10, -10, -10, -10, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -11, -11, -11, -11, -11, -11, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -13, -13, -13, -13, -13, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -3, -3, -3, -3, -3, -3, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, -22, 0, 0, -22, 0,
        // State 35
        0, 44, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, 0, -23, 0, 0, -23, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, -21, 0, 0, -21, 0,
        // State 38
        0, -5, 10, -5, -5, 11, -5, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -6, 10, -6, -6, 11, -6, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 7, 8, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -8, -8, -8, -8, -8, -8, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -9, -9, -9, -9, -9, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 17 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -18,
        // State 1
        -19,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -20,
        // State 12
        0,
        // State 13
        -32,
        // State 14
        -26,
        // State 15
        -4,
        // State 16
        0,
        // State 17
        -27,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -22,
        // State 35
        0,
        // State 36
        -23,
        // State 37
        -21,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                3 => 32,
                4 => 33,
                _ => 18,
            },
            1 => 19,
            2 => 11,
            3 => match state {
                5 => 35,
                8 => 40,
                _ => 20,
            },
            4 => match state {
                6 => 38,
                7 => 39,
                _ => 21,
            },
            5 => 22,
            6 => match state {
                2..=10 => 23,
                _ => 12,
            },
            7 => 24,
            8 => 25,
            9 => 13,
            10 => match state {
                1 => 17,
                _ => 14,
            },
            12 => 1,
            13 => 26,
            14 => match state {
                9 => 41,
                10 => 42,
                _ => 27,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""=""###,
            r###""==""###,
            r###""assert""###,
            r###""print""###,
            r###"r#"\"[^\"]+\""#"###,
            r###"r#"//[^\\n\\r]*[\\n\\r]*"#"###,
            r###"r#"[+-]?([0-9]*\\.[0-9]+)|([0-9]+\\.[0-9]*)"#"###,
            r###"r#"[+-]?[0-9]+"#"###,
            r###"r#"[a-z_][a-z0-9_]*"#"###,
            r###"r#"true|false"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, 'a>
    where 'input: 'a
    {
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'a ())>,
    }
    impl<'input, 'a> __state_machine::ParserDefinition for __StateMachine<'input, 'a>
    where 'input: 'a
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = alloc::vec::Vec<()>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 17 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.state,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
        'a,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(5, _) if true => Some(0),
            Token(6, _) if true => Some(1),
            Token(7, _) if true => Some(2),
            Token(8, _) if true => Some(3),
            Token(9, _) if true => Some(4),
            Token(10, _) if true => Some(5),
            Token(11, _) if true => Some(6),
            Token(12, _) if true => Some(7),
            Token(13, _) if true => Some(8),
            Token(14, _) if true => Some(9),
            Token(15, _) if true => Some(10),
            Token(0, _) if true => Some(11),
            Token(1, _) if true => Some(12),
            Token(2, _) if true => Some(13),
            Token(3, _) if true => Some(14),
            Token(4, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'a,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => match __token {
                Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(16, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ScriptParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ScriptParser {
        pub fn new() -> ScriptParser {
            let __builder = super::__intern_token::new_builder();
            ScriptParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'a,
        >(
            &self,
            state: &'a mut HashMap<&'input str, Variable>,
            input: &'input str,
        ) -> Result<alloc::vec::Vec<()>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    state,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> Option<Result<alloc::vec::Vec<()>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(state, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                // __Script = Script => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(state, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Variable, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<()>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // BoolExpression = Expression, "==", Expression => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // BoolExpression = Expression => ActionFn(20);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Boolean = r#"true|false"# => ActionFn(2);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Comment = r#"//[^\\n\\r]*[\\n\\r]*"# => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "+", Factor => ActionFn(16);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce5<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "-", Factor => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Expression = Factor => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce7<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Term => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Term => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce9<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce10<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Float = r#"[+-]?([0-9]*\\.[0-9]+)|([0-9]+\\.[0-9]*)"# => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce11<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Identifier = r#"[a-z_][a-z0-9_]*"# => ActionFn(1);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Integer = r#"[+-]?[0-9]+"# => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(6);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Literal = Integer => ActionFn(7);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce15<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Literal = Float => ActionFn(8);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Literal = String => ActionFn(9);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce17<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Script =  => ActionFn(31);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action31::<>(state, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Script = Statement+ => ActionFn(32);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement = Comment => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement = Identifier, "=", BoolExpression, ";" => ActionFn(23);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action23::<>(state, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 10)
    }
    pub(crate) fn __reduce21<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement = "assert", BoolExpression, ";" => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce22<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement = "print", BoolExpression, ";" => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce23<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(27);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action27::<>(state, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce24<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(28);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(state, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce27<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // String = r#"\"[^\"]+\""# => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(10);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce29<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(state, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce30<
        'input,
        'a,
    >(
        state: &'a mut HashMap<&'input str, Variable>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'a ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(state, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 14)
    }
}
pub use self::__parse__Script::ScriptParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::ast::Variable;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(\"[\u{0}-!\\#-\u{10ffff}]+\")", false),
            ("^(//[\u{0}-\t\u{b}-\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", false),
            ("^([\\+\\-]?([0-9]*\\.[0-9]+)|([0-9]+\\.[0-9]*))", false),
            ("^([\\+\\-]?[0-9]+)", false),
            ("^([_a-z][0-9_a-z]*)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(;)", false),
            ("^(=)", false),
            ("^(==)", false),
            ("^(assert)", false),
            ("^(print)", false),
            ("^(true|false)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<()>, usize),
) -> alloc::vec::Vec<()>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    bool::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    s.trim_matches('"').to_string()
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, b, _): (usize, bool, usize),
) -> Variable
{
    Variable::Boolean(b)
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, i, _): (usize, i64, usize),
) -> Variable
{
    Variable::Integer(i)
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, f, _): (usize, f64, usize),
) -> Variable
{
    Variable::Float(f)
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, s, _): (usize, String, usize),
) -> Variable
{
    Variable::String(s)
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, Variable, usize),
) -> Variable
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
) -> Variable
{
    state.get(i).unwrap().clone()
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Variable
{
    __0
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, l, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Variable, usize),
) -> Variable
{
    l.multiply(r)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, l, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Variable, usize),
) -> Variable
{
    l.divide(r)
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, Variable, usize),
) -> Variable
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, l, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Variable, usize),
) -> Variable
{
    l.add(r)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, l, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Variable, usize),
) -> Variable
{
    l.subtract(r)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, Variable, usize),
) -> Variable
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, l, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Variable, usize),
) -> Variable
{
    l.equals(r)
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, Variable, usize),
) -> Variable
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    {}
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    { state.insert(i, v); }
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        match v {
            Variable::Boolean(v) => assert!(v),
            _ => todo!(),
        }
    }
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Variable, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    println!("{}", v)
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<()>, usize),
) -> alloc::vec::Vec<()>
{
    __0
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<()>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<()>, usize),
) -> alloc::vec::Vec<()>
{
    v
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> alloc::vec::Vec<()>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<()>, usize),
    (_, e, _): (usize, (), usize),
) -> alloc::vec::Vec<()>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<()>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action27(
        state,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        state,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'a,
>(
    state: &'a mut HashMap<&'input str, Variable>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<()>, usize),
) -> alloc::vec::Vec<()>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action28(
        state,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        state,
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'a, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, 'a, > __ToTriple<'input, 'a, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, 'a, > __ToTriple<'input, 'a, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
