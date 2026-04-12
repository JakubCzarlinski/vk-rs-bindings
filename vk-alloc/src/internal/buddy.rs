use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct RangeAllocator {
    free_ranges: BTreeMap<u64, u64>,
}

impl RangeAllocator {
    pub fn new(size: u64) -> Self {
        let mut free_ranges = BTreeMap::new();
        free_ranges.insert(0, size);
        Self { free_ranges }
    }

    pub fn allocate(&mut self, size: u64, alignment: u64) -> Option<u64> {
        let alignment = alignment.max(1);
        let candidate = self.free_ranges.iter().find_map(|(&offset, &len)| {
            let aligned = align_up(offset, alignment);
            let padding = aligned - offset;
            if len >= size + padding {
                Some((offset, len, aligned))
            } else {
                None
            }
        })?;

        self.free_ranges.remove(&candidate.0);
        if candidate.2 > candidate.0 {
            self.free_ranges
                .insert(candidate.0, candidate.2 - candidate.0);
        }
        let end = candidate.2 + size;
        let candidate_end = candidate.0 + candidate.1;
        if end < candidate_end {
            self.free_ranges.insert(end, candidate_end - end);
        }
        Some(candidate.2)
    }

    pub fn free(&mut self, offset: u64, size: u64) {
        let mut start = offset;
        let mut len = size;

        if let Some((&prev_start, &prev_len)) = self.free_ranges.range(..=offset).next_back()
            && prev_start + prev_len == offset
        {
            start = prev_start;
            len += prev_len;
            self.free_ranges.remove(&prev_start);
        }
        if let Some((&next_start, &next_len)) = self.free_ranges.range(offset..).next()
            && offset + size == next_start
        {
            len += next_len;
            self.free_ranges.remove(&next_start);
        }
        self.free_ranges.insert(start, len);
    }

    #[cfg_attr(not(any(test, feature = "bench-internals")), allow(dead_code))]
    pub fn is_non_overlapping(&self) -> bool {
        let mut prev_end = 0;
        let mut first = true;
        for (&start, &len) in &self.free_ranges {
            if !first && start < prev_end {
                return false;
            }
            prev_end = start + len;
            first = false;
        }
        true
    }
}

pub const fn align_up(value: u64, alignment: u64) -> u64 {
    if alignment <= 1 {
        value
    } else {
        let rem = value % alignment;
        if rem == 0 {
            value
        } else {
            value + (alignment - rem)
        }
    }
}
