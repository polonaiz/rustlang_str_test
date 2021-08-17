
#[test]
fn test_quote() {
	let haystack = "aaa,bbb,'ccc,ddd',eee";
	let mut flag_qouted = false;
	haystack
		.split(|c| match c {
			'\'' => {
				flag_qouted = !flag_qouted;
				false
			}
			',' if flag_qouted => false,
			',' if !flag_qouted => true,
			_ => false,
		})
		.for_each(|s: &str| {
			let p = s.as_ptr();
			println!("@{:?} {}", p, &s);
		});
}

#[test]
fn test_fill() {
	let haystack_source = "a,bbb,c,dd,eee,f,gg";
	let mut haystack_feeder = std::io::Cursor::new(haystack_source);

	// initial state
	const MAX_TOKEN_SIZE: usize = 4;
	let mut haystack_buf = [0u8; MAX_TOKEN_SIZE];
	unsafe {
		println!(
			"haystack_buf {:02x?} @{:?}..{:?}",
			&haystack_buf,
			&haystack_buf.as_ptr(),
			&haystack_buf.as_ptr().offset(MAX_TOKEN_SIZE as isize),
		);
	}

	let mut feed_offset = 0;
	loop {
		println!("feed_offset {}", feed_offset);
		// feed
		use std::io::Read;
		let feed_size = haystack_feeder
			.read(&mut haystack_buf[feed_offset..])
			.unwrap();
		println!("feed_size {}", feed_size);
		if feed_size == 0 {
			println!("EOF");
			break;
		}
		unsafe {
			println!(
				"haystack_buf {:02x?} @{:?}..{:?}",
				&haystack_buf,
				&haystack_buf.as_ptr(),
				&haystack_buf.as_ptr().offset(MAX_TOKEN_SIZE as isize),
			);
		}
		let haystack_len = feed_offset + feed_size;
		let haystack = &haystack_buf[..haystack_len];
		unsafe {
			println!(
				"haystack \"{}\" {:02x?} @{:?}..{:?}",
				&std::str::from_utf8(&haystack).unwrap(),
				&haystack,
				&haystack.as_ptr(),
				&haystack.as_ptr().offset(haystack_len as isize),
			);
		}

		// setup
		let mut last_token_buf = [0u8; MAX_TOKEN_SIZE];
		let last_token_len;
		let splitter = |c| c == ','; // to be replaced with closure!!
		{
			let mut last_token_str: &str = "";
			std::str::from_utf8(&haystack)
				.unwrap()
				.split_inclusive(splitter)
				.for_each(|token: &str| unsafe {
					println!(
						"token \"{}\" @{:?}..{:?}",
						&token,
						token.as_ptr(),
						token.as_ptr().offset(token.len() as isize),
					);
					last_token_str = token;
				});
			unsafe {
				std::ptr::copy(
					last_token_str.as_ptr(),
					last_token_buf.as_mut_ptr(),
					last_token_str.len(),
				);
			}
			last_token_len = last_token_str.len();
		}

		let last_token = std::str::from_utf8(&last_token_buf[..last_token_len]).unwrap();
		let last_char: char = last_token.chars().rev().next().unwrap();
		let is_token_partted = !splitter(last_char);
		if is_token_partted {
			assert!(last_token.len() < MAX_TOKEN_SIZE);
			unsafe {
				std::ptr::copy(
					last_token.as_ptr(),
					haystack_buf.as_mut_ptr(),
					last_token.len(),
				);
			}
			feed_offset = last_token_len;
			println!("partted token, copied to front!");
		} else {
			feed_offset = 0;
		}
	}
}

// haystack_buf [00, 00, 00, 00] @0x7fd2c9f062fc..0x7fd2c9f06300
// feed_offset 0
// feed_size 4
// haystack_buf [61, 2c, 62, 62] @0x7fd2c9f062fc..0x7fd2c9f06300
// haystack "a,bb" [61, 2c, 62, 62] @0x7fd2c9f062fc..0x7fd2c9f06300
// token "a," @0x7fd2c9f062fc..0x7fd2c9f062fe
// token "bb" @0x7fd2c9f062fe..0x7fd2c9f06300
// partted token, copied to front!
// feed_offset 2
// feed_size 2
// haystack_buf [62, 62, 62, 2c] @0x7fd2c9f062fc..0x7fd2c9f06300
// haystack "bbb," [62, 62, 62, 2c] @0x7fd2c9f062fc..0x7fd2c9f06300
// token "bbb," @0x7fd2c9f062fc..0x7fd2c9f06300
// feed_offset 0
// feed_size 4
// haystack_buf [63, 2c, 64, 64] @0x7fd2c9f062fc..0x7fd2c9f06300
// haystack "c,dd" [63, 2c, 64, 64] @0x7fd2c9f062fc..0x7fd2c9f06300
// token "c," @0x7fd2c9f062fc..0x7fd2c9f062fe
// token "dd" @0x7fd2c9f062fe..0x7fd2c9f06300
// partted token, copied to front!
// feed_offset 2
// feed_size 2
// haystack_buf [64, 64, 2c, 65] @0x7fd2c9f062fc..0x7fd2c9f06300
// haystack "dd,e" [64, 64, 2c, 65] @0x7fd2c9f062fc..0x7fd2c9f06300
// token "dd," @0x7fd2c9f062fc..0x7fd2c9f062ff
// token "e" @0x7fd2c9f062ff..0x7fd2c9f06300
// partted token, copied to front!
// feed_offset 1
// feed_size 3
// haystack_buf [65, 65, 65, 2c] @0x7fd2c9f062fc..0x7fd2c9f06300
// haystack "eee," [65, 65, 65, 2c] @0x7fd2c9f062fc..0x7fd2c9f06300
// token "eee," @0x7fd2c9f062fc..0x7fd2c9f06300
// feed_offset 0
// feed_size 4
// haystack_buf [66, 2c, 67, 67] @0x7fd2c9f062fc..0x7fd2c9f06300
// haystack "f,gg" [66, 2c, 67, 67] @0x7fd2c9f062fc..0x7fd2c9f06300
// token "f," @0x7fd2c9f062fc..0x7fd2c9f062fe
// token "gg" @0x7fd2c9f062fe..0x7fd2c9f06300
// partted token, copied to front!
// feed_offset 2
// feed_size 0
// EOF
