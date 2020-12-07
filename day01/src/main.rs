const INPUTS: [u64; 200] = [
	1630, 1801, 1917, 1958, 1953, 1521, 1990, 1959, 1543, 1798, 638, 1499, 1977, 1433, 1532, 1780,
	1559, 1866, 1962, 1999, 1623, 1772, 1730, 1670, 1791, 1947, 1961, 1523, 959, 1998, 1693, 1490,
	1712, 910, 1635, 1837, 586, 1590, 1741, 1739, 1660, 1883, 1777, 1734, 1413, 1456, 1511, 1957,
	1738, 1685, 1677, 1419, 1566, 1639, 1578, 1922, 1856, 1946, 1965, 1649, 1854, 1610, 1806, 1424,
	1616, 218, 1678, 1992, 1985, 903, 1626, 1412, 1964, 671, 1692, 1571, 1690, 1587, 1933, 1367,
	1585, 1575, 498, 1601, 2005, 1711, 1948, 1991, 1580, 1704, 207, 1560, 1867, 1600, 1594, 1930,
	1541, 1832, 1613, 1599, 1757, 71, 1534, 1940, 1982, 1960, 1530, 1908, 1857, 1410, 1987, 1526,
	1546, 2002, 1923, 1972, 1752, 1984, 1754, 1916, 1942, 1980, 1608, 1398, 1438, 1955, 1968, 1799,
	1976, 1847, 1775, 1904, 1983, 1945, 1554, 1486, 1527, 1884, 1553, 1736, 1561, 1513, 1695, 1431,
	1997, 1405, 1872, 1434, 1679, 1609, 105, 1582, 1795, 1826, 1886, 1472, 2007, 1617, 1978, 1669,
	1764, 1865, 1773, 1993, 1666, 1583, 2009, 1969, 2001, 1659, 1833, 1713, 1893, 2000, 1520, 1652,
	1437, 1556, 1633, 1386, 1819, 1973, 1426, 1975, 2010, 1863, 1593, 1996, 1796, 1986, 1995, 657,
	1784, 1644, 1941, 1596, 1849, 1065, 1927, 1525
];

const TARGET_SUM: u64 = 2020;

fn main() {
	if let Some((v1, v2)) = two_vals(INPUTS.to_vec(), TARGET_SUM) {
		println!("The sum of ({}, {}): {}", v1, v2, v1 + v2);
		println!("The product of ({}, {}): {}", v1, v2, v1 * v2);
	} else {
		println!("No suitable result found.");
	}

	if let Some((v1, v2, v3)) = three_vals(INPUTS.to_vec(), TARGET_SUM) {
		println!("The sum of ({}, {}, {}): {}", v1, v2, v3, v1 + v2 + v3);
		println!("The product of ({}, {}, {}): {}", v1, v2, v3, v1 * v2 * v3);
	} else {
		println!("No suitable result found.");
	}
}

fn two_vals(inputs: Vec<u64>, target_sum: u64) -> Option<(u64, u64)> {
	let mut ans: Option<(u64, u64)> = None;

	inputs.iter().enumerate().for_each(|(i1, v1)| {
		inputs.iter().enumerate().for_each(|(i2, v2)| {
			// check if answer have already been found
			if ans.is_some() { return }
			// we exclude indexing the same value twice
			if i1 == i2 { return }
			// this is not what we want
			if v1 + v2 != target_sum { return }

			ans = Some((*v1, *v2));
		});
	});

	ans
}

fn three_vals(inputs: Vec<u64>, target_sum: u64) -> Option<(u64, u64, u64)> {
	let mut ans: Option<(u64, u64, u64)> = None;

	inputs.iter().enumerate().for_each(|(i1, v1)| {
		inputs.iter().enumerate().for_each(|(i2, v2)| {
			inputs.iter().enumerate().for_each(|(i3, v3)| {
				// check if answer have already been found
				if ans.is_some() { return }
				// we exclude indexing the same value twice
				if i1 == i2 || i2 == i3 { return }
				// this is not what we want
				if v1 + v2 + v3 != target_sum { return }

				ans = Some((*v1, *v2, *v3));
			});
		});
	});

	ans
}
