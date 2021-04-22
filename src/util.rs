pub enum Argument<'a> {
	Short(char),
	ShortValue(char, &'a str),
	Long(&'a str),
	LongValue(&'a str, &'a str),
	Normal(&'a str)
}

enum SpecialState<'a> {
	LastShortContinue(&'a str),
	SwitchesEnd,
	None
}

pub struct ArgumentParser<'a, I, F>
		where I: Iterator<Item = &'a str>,
			F: FnMut(bool, &str) -> Option<(&str, &str)> {
	state: SpecialState<'a>,
	value_extractor: F,
	arguments: I
}

impl<'a, I, F> ArgumentParser<'a, I, F>
		where I: Iterator<Item = &'a str>,
			F: FnMut(bool, &str) -> Option<(&str, &str)> {
	pub fn new_extractor<N>(arguments: N, value_extractor: F) -> Self
			where N: IntoIterator<IntoIter = I> {
		Self {
			state: SpecialState::None,
			arguments: arguments.into_iter(),
			value_extractor
		}
	}
}

/*
impl<'a, I> ArgumentParser<'a, I, fn(bool, &str) -> Option<(&str, &str)>>
		where I: Iterator<Item = &'a str> {
	pub fn default_extractor(long: bool, content: &str) -> Option<(&str, &str)> {
		match long {
			true => {
				content.find("=")
			}
		}
	}
			
	pub fn new<N>(arguments: N) -> Self
			where N: IntoIterator<IntoIter = I> {
		Self::new_extractor(arguments, Self::default_extractor)
	}
}*/

impl<'a, I, F> Iterator for ArgumentParser<'a, I, F>
		where I: Iterator<Item = &'a str>,
			F: FnMut(bool, &str) -> Option<(&str, &str)> {
	type Item = Argument<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		Some(match &self.state {
			SpecialState::None => {
				let arg = self.arguments.next()?;
				
				match arg.split_at(1) {
					("-", arg) => match arg.split_at(1) {
						("-", arg) => match arg {
							"" => { // --
								self.state = SpecialState::SwitchesEnd;
								self.next()?
							},
							_ => match (self.value_extractor)(true, arg) {
								// --name=value
								Some((name, value)) => Argument::LongValue(name, value),
								// --name
								None => Argument::Long(arg)
							}
						},
						_ => match (self.value_extractor)(false, arg) {
							// -n=value
							Some((name, value)) => {
								let mut chars = name.chars();
								let name = chars.next().expect("error");
								debug_assert_eq!(None, chars.next());

								Argument::ShortValue(name, value)
							},
							// -n
							None => {
								let mut chars = arg.char_indices();
								let name = chars.next().expect("error").1;
								if let Some((rest, _)) = chars.next() {
									self.state = SpecialState::LastShortContinue(&arg[rest..]);
								}

								Argument::Short(name)
							}
						}
					},
					// value
					_ => Argument::Normal(arg)
				}
			},
			// -- value
			SpecialState::SwitchesEnd => Argument::Normal(self.arguments.next()?),
			SpecialState::LastShortContinue(arg) =>
					match (self.value_extractor)(false, arg) {
				// -on=value
				Some((name, value)) => {
					let mut chars = name.chars();
					let name = chars.next().expect("error");
					debug_assert_eq!(None, chars.next());

					self.state = SpecialState::None;
					Argument::ShortValue(name, value)
				},
				// -on
				None => {
					let mut chars = arg.char_indices();
					let name = chars.next().expect("error").1;
					self.state = chars.next()
						.map(|(rest, _)| SpecialState::LastShortContinue(&arg[rest..]))
						.unwrap_or(SpecialState::None);

					Argument::Short(name)
				}
			}
		})
	}
}
