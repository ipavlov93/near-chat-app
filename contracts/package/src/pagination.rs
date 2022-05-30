//! The package provides utils methods than can be used by contracts.

/// # Description
/// Calculates start and final element indexes
/// # Params
/// * **page** is a page number of type [`u64`]
/// * **limit** is a limit number per page of type [`u64`]
pub fn calculate_pagination_range(page: u64, limit: u64) -> (u64, u64) {
    let mut offset = 0;
    if page > 1 {
        offset = (page - 1) * limit;
    }

    (offset, offset + limit)
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate() {
        let limit = 100;
        assert_eq!(calculate_pagination_range(0, limit), (0, limit));
        assert_eq!(calculate_pagination_range(1, limit), (0, limit));
        assert_eq!(calculate_pagination_range(2, limit), (100, 200));
    }
}
