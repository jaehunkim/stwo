use core::ops::Sub;
use std::ops::{Add, Mul};

use crate::core::backend::simd::SimdBackend;
use crate::core::backend::wgpu::WgpuBackend;
use crate::core::fields::m31::BaseField;
use crate::core::fields::qm31::SecureField;
use crate::core::lookups::mle::{Mle, MleOps};

impl MleOps<BaseField> for WgpuBackend {
    fn fix_first_variable(
        mle: Mle<Self, BaseField>,
        assignment: SecureField,
    ) -> Mle<Self, SecureField> {
        SimdBackend::fix_first_variable(mle, assignment)
    }
}

impl MleOps<SecureField> for WgpuBackend {
    fn fix_first_variable(
        mle: Mle<Self, SecureField>,
        assignment: SecureField,
    ) -> Mle<Self, SecureField> {
        SimdBackend::fix_first_variable(mle, assignment)
    }
}

fn fold_packed_mle_evals<
    PackedF: Sub<Output = PackedF> + Copy,
    PackedEF: Mul<PackedF, Output = PackedEF> + Add<PackedF, Output = PackedEF>,
>(
    assignment: PackedEF,
    eval0: PackedF,
    eval1: PackedF,
) -> PackedEF {
    SimdBackend::fold_packed_mle_evals(assignment, eval0, eval1)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::core::backend::wgpu::WgpuBackend;
    use crate::core::backend::{Column, CpuBackend};
    use crate::core::channel::Channel;
    use crate::core::fields::m31::BaseField;
    use crate::core::fields::qm31::SecureField;
    use crate::core::lookups::mle::Mle;
    use crate::core::test_utils::test_channel;

    #[test]
    fn fix_first_variable_with_secure_field_mle_matches_cpu() {
        const N_VARIABLES: u32 = 8;
        let values = test_channel().draw_felts(1 << N_VARIABLES);
        let mle_simd = Mle::<WgpuBackend, SecureField>::new(values.iter().copied().collect());
        let mle_cpu = Mle::<CpuBackend, SecureField>::new(values);
        let random_assignment = SecureField::from_u32_unchecked(7, 12, 3, 2);
        let mle_fixed_cpu = mle_cpu.fix_first_variable(random_assignment);

        let mle_fixed_simd = mle_simd.fix_first_variable(random_assignment);

        assert_eq!(mle_fixed_simd.into_evals().to_cpu(), *mle_fixed_cpu)
    }

    #[test]
    fn fix_first_variable_with_base_field_mle_matches_cpu() {
        const N_VARIABLES: u32 = 8;
        let values = (0..1 << N_VARIABLES).map(BaseField::from).collect_vec();
        let mle_simd = Mle::<WgpuBackend, BaseField>::new(values.iter().copied().collect());
        let mle_cpu = Mle::<CpuBackend, BaseField>::new(values);
        let random_assignment = SecureField::from_u32_unchecked(7, 12, 3, 2);
        let mle_fixed_cpu = mle_cpu.fix_first_variable(random_assignment);

        let mle_fixed_simd = mle_simd.fix_first_variable(random_assignment);

        assert_eq!(mle_fixed_simd.into_evals().to_cpu(), *mle_fixed_cpu)
    }
}
