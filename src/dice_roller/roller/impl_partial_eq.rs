impl PartialEq for super::Roller {
    fn eq(&self, other: &super::Roller) -> bool {
        self.dice == other.dice
            && self.sides == other.sides
            && self.modifier == other.modifier
            && self.success_threshold == other.success_threshold
            && self.explode_threshold == other.explode_threshold
            && self.take_max == other.take_max
            && self.take_min == other.take_min
            && self.take_mid == other.take_mid
    }
}
