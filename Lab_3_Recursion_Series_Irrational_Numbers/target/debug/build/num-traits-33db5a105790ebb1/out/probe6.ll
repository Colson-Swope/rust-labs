; ModuleID = 'probe6.f5beb7b6-cgu.0'
source_filename = "probe6.f5beb7b6-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h6b1540dc2d6b477cE"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca i8, align 1
  %0 = alloca i8, align 1
; call core::f64::<impl f64>::classify
  %1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17h99899615d04ac55dE"(double %self), !range !2
  store i8 %1, i8* %_2, align 1
  br label %bb1

bb1:                                              ; preds = %start
  %2 = load i8, i8* %_2, align 1, !range !2, !noundef !3
  %_4 = zext i8 %2 to i64
  %3 = icmp eq i64 %_4, 3
  br i1 %3, label %bb3, label %bb2

bb3:                                              ; preds = %bb1
  store i8 1, i8* %0, align 1
  br label %bb4

bb2:                                              ; preds = %bb1
  store i8 0, i8* %0, align 1
  br label %bb4

bb4:                                              ; preds = %bb3, %bb2
  %4 = load i8, i8* %0, align 1, !range !4, !noundef !3
  %5 = trunc i8 %4 to i1
  ret i1 %5
}

; probe6::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe65probe17h446e524b78dbfe9fE() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h6b1540dc2d6b477cE"(double 1.000000e+00)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::f64::<impl f64>::classify
; Function Attrs: nonlazybind uwtable
declare i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17h99899615d04ac55dE"(double) unnamed_addr #1

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i8 0, i8 5}
!3 = !{}
!4 = !{i8 0, i8 2}
