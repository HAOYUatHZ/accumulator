/// See https://bheisler.github.io/criterion.rs/book/getting_started.html to add more benchmarks.
#[macro_use]
extern crate criterion;

use accumulator::group::{ClassGroup, ElemFrom, Group, UnknownOrderGroup};
use criterion::Criterion;
use rug::Integer;
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
  let left = ClassGroup::elem((
    Integer::from_str("16").unwrap(),
    Integer::from_str("9").unwrap(),
    Integer::from_str(
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341\
       018110672047217112377476473502060121352842575308793237621563947157630098485131517401073\
       775191194319531549483898334742144138601661120476425524333273122132151927833887323969998\
       955713328783526854198871332313399489386997681827578317938792170918711794684859311697439\
       726596656501594138449739494228617068329664776714484742276158090583495714649193839084110\
       987149118615158361352488488402038894799695420483272708933239751363849397287571692736881\
       031223140446926522431859701738994562905746276604714085486912447322113758834733508155518\
       6814036",
    )
    .unwrap(),
  ));
  let right = left.clone();

  // generator
  let base = ClassGroup::elem((
    Integer::from(2),
    Integer::from(1),
    Integer::from_str(
      "382700862935094049338670718954010190193660954702063348783962358222530000466648930602728\
       1448853763777368990198117880164809708227406024703459009725115772610407878810521392085902015\
       2955455625239587118667793715310881328896381140419466618497705721542267109859175999164570663\
       0268214833590970658507195915095981454620626543510337367349694357478874493579517812773252012\
       7531075979159538289365466373182137158779382092647246679657171935507126728878971929489212668\
       9081990790721631115839756336386618167146591801091079517830057354189504824978512357541217945\
       48776139119565032459702128377126838952995785769100706778680652441494512278",
    )
    .unwrap(),
  ));
  let exp = Integer::from_str("65315").unwrap();
  let g_inv = base.clone();
  let g_sq = ClassGroup::unknown_order_elem();

  let aa = Integer::from_str("16").unwrap();
  let bb = Integer::from_str("105").unwrap();
  let cc = Integer::from_str(
    "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
     672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
     531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
     198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
     494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
     038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
     9057462766047140854869124473221137588347335081555186814207",
  )
  .unwrap();

  // element which requires one iteration to reduce
  let g_red = <ClassGroup as Group>::Elem::new_raw(cc.clone(), bb.clone(), aa.clone());
  let g_norm = <ClassGroup as Group>::Elem::new_raw(aa.clone(), bb.clone(), cc.clone());

  c.bench_function("group_class_op", move |b| {
    b.iter(|| ClassGroup::op(&left, &right))
  });
  c.bench_function("group_class_exp", move |b| {
    b.iter(|| ClassGroup::exp(&base, &exp))
  });
  c.bench_function("group_class_inv", move |b| {
    b.iter(|| ClassGroup::inv(&g_inv))
  });
  c.bench_function("group_class_normalize", move |b| {
    b.iter_with_setup(
      || g_norm.clone(),
      |g| <ClassGroup as Group>::Elem::normalize_pub(g),
    )
  });
  c.bench_function("group_class_reduce", move |b| {
    b.iter_with_setup(
      || g_red.clone(),
      |g| <ClassGroup as Group>::Elem::reduce_pub(g),
    )
  });
  c.bench_function("group_class_square", move |b| {
    b.iter_with_setup(|| g_sq.clone(), |mut g| g.square_pub())
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
