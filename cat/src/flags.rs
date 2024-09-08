use std::ops::BitOr;
use bitflags::bitflags;
use crate::Args;

bitflags! {
	#[derive(Copy, PartialEq, Clone, Debug)]
	pub struct Flags: u32 {
		const NUMBER_NONBLANK = 0b0000_0010;
		const SHOW_NONPRINTING = 0b0000_0001;
		const VE = 0b0000_0100;
		const SHOW_ENDS = 0b0000_1000;
		const NUMBER = 0b0001_0000;
		const SQUEEZE_BLANK = 0b0010_0000;
		const SHOW_TABS = 0b1000_0000;
	}
}

impl Flags {

	fn set_showall(&mut self, show_all: bool) {
		if show_all {
			*self |= Flags::SHOW_NONPRINTING;
			*self |= Flags::SHOW_TABS;
			*self |= Flags::SHOW_ENDS;
		}
	}

	fn set_nonblank(&mut self, nonblank: bool) {
		if nonblank {
			*self |= Flags::NUMBER_NONBLANK;
			if (self.contains(Flags::NUMBER)) {
				*self &= !Flags::NUMBER;
			}
		}
	}

	fn set_ve(&mut self, ve: bool) {
		if ve {
			*self |= Flags::SHOW_NONPRINTING;
			*self |= Flags::SHOW_ENDS;
		}
	}

	fn set_number(&mut self, number: bool) {
		if number {
			*self |= Flags:: NUMBER;
		}
	}

	fn set_squeeze_blank(&mut self, squeeze_blank: bool) {
		if squeeze_blank {
			*self |= Flags::SQUEEZE_BLANK;
		}
	}

	fn set_show_ends(&mut self, show_ends: bool) {
		if show_ends {
			*self |= Flags::SHOW_ENDS;
		}
	}

	fn set_vt(&mut self, vt: bool) {
		if vt {
			*self |= Flags::SHOW_NONPRINTING;
			*self |= Flags::SHOW_TABS;
		}
	}

	fn set_show_tabs(&mut self, show_tabs: bool) {
		if show_tabs {
			*self |= Flags::SHOW_TABS;
		}
	}

	fn set_non_printing(&mut self, nonprinting: bool) {
		if nonprinting {
			*self |= Flags::SHOW_NONPRINTING;
		}
	}
}
impl From<Args> for Flags {
	fn from(args: Args) -> Self {
		let mut flags = Flags::empty();

		flags.set_showall(args.show_all);
		flags.set_number(args.number);
		flags.set_ve(args.ve);
		flags.set_show_ends(args.show_ends);
		flags.set_nonblank(args.number_nonblank);
		flags.set_squeeze_blank(args.squeeze_blank);
		flags.set_vt(args.vt);
		flags.set_show_tabs(args.show_tabs);
		flags.set_non_printing(args.show_nonprinting);

		flags
	}
}

mod tests {
	use crate::flags::Flags;

	#[test]
	fn test_flag_setting() {
		let mut flags = Flags::empty();

		flags.set_showall(true);
		assert!(flags.contains(Flags::SHOW_NONPRINTING));
		assert!(flags.contains(Flags::SHOW_TABS));
		assert!(flags.contains(Flags::SHOW_ENDS));

		flags.set_number(true);
		assert!(flags.contains(Flags::NUMBER));

		flags.set_nonblank(true);
		assert!(flags.contains(Flags::NUMBER_NONBLANK));
		assert!(!flags.contains(Flags::NUMBER));

		flags.set_ignored(true);
		assert!(flags.contains(Flags::IGNORED));
	}
}
