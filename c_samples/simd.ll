; ModuleID = 'simd.c'
source_filename = "simd.c"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.26.28806"

; Function Attrs: norecurse nounwind readonly uwtable
define dso_local i32 @sumint(i32* nocapture readonly %0, i32 %1) local_unnamed_addr #0 {
  %3 = icmp sgt i32 %1, 0
  br i1 %3, label %4, label %106

4:                                                ; preds = %2
  %5 = zext i32 %1 to i64
  %6 = icmp ult i32 %1, 32
  br i1 %6, label %7, label %10

7:                                                ; preds = %90, %4
  %8 = phi i64 [ 0, %4 ], [ %11, %90 ]
  %9 = phi i32 [ 0, %4 ], [ %104, %90 ]
  br label %108

10:                                               ; preds = %4
  %11 = and i64 %5, 4294967264
  %12 = add nsw i64 %11, -32
  %13 = lshr exact i64 %12, 5
  %14 = add nuw nsw i64 %13, 1
  %15 = and i64 %14, 1
  %16 = icmp eq i64 %12, 0
  br i1 %16, label %62, label %17

17:                                               ; preds = %10
  %18 = sub nuw nsw i64 %14, %15
  br label %19

19:                                               ; preds = %19, %17
  %20 = phi i64 [ 0, %17 ], [ %59, %19 ]
  %21 = phi <8 x i32> [ zeroinitializer, %17 ], [ %55, %19 ]
  %22 = phi <8 x i32> [ zeroinitializer, %17 ], [ %56, %19 ]
  %23 = phi <8 x i32> [ zeroinitializer, %17 ], [ %57, %19 ]
  %24 = phi <8 x i32> [ zeroinitializer, %17 ], [ %58, %19 ]
  %25 = phi i64 [ %18, %17 ], [ %60, %19 ]
  %26 = getelementptr inbounds i32, i32* %0, i64 %20
  %27 = bitcast i32* %26 to <8 x i32>*
  %28 = load <8 x i32>, <8 x i32>* %27, align 4, !tbaa !3
  %29 = getelementptr inbounds i32, i32* %26, i64 8
  %30 = bitcast i32* %29 to <8 x i32>*
  %31 = load <8 x i32>, <8 x i32>* %30, align 4, !tbaa !3
  %32 = getelementptr inbounds i32, i32* %26, i64 16
  %33 = bitcast i32* %32 to <8 x i32>*
  %34 = load <8 x i32>, <8 x i32>* %33, align 4, !tbaa !3
  %35 = getelementptr inbounds i32, i32* %26, i64 24
  %36 = bitcast i32* %35 to <8 x i32>*
  %37 = load <8 x i32>, <8 x i32>* %36, align 4, !tbaa !3
  %38 = add <8 x i32> %28, %21
  %39 = add <8 x i32> %31, %22
  %40 = add <8 x i32> %34, %23
  %41 = add <8 x i32> %37, %24
  %42 = or i64 %20, 32
  %43 = getelementptr inbounds i32, i32* %0, i64 %42
  %44 = bitcast i32* %43 to <8 x i32>*
  %45 = load <8 x i32>, <8 x i32>* %44, align 4, !tbaa !3
  %46 = getelementptr inbounds i32, i32* %43, i64 8
  %47 = bitcast i32* %46 to <8 x i32>*
  %48 = load <8 x i32>, <8 x i32>* %47, align 4, !tbaa !3
  %49 = getelementptr inbounds i32, i32* %43, i64 16
  %50 = bitcast i32* %49 to <8 x i32>*
  %51 = load <8 x i32>, <8 x i32>* %50, align 4, !tbaa !3
  %52 = getelementptr inbounds i32, i32* %43, i64 24
  %53 = bitcast i32* %52 to <8 x i32>*
  %54 = load <8 x i32>, <8 x i32>* %53, align 4, !tbaa !3
  %55 = add <8 x i32> %45, %38
  %56 = add <8 x i32> %48, %39
  %57 = add <8 x i32> %51, %40
  %58 = add <8 x i32> %54, %41
  %59 = add i64 %20, 64
  %60 = add i64 %25, -2
  %61 = icmp eq i64 %60, 0
  br i1 %61, label %62, label %19, !llvm.loop !7

62:                                               ; preds = %19, %10
  %63 = phi <8 x i32> [ undef, %10 ], [ %55, %19 ]
  %64 = phi <8 x i32> [ undef, %10 ], [ %56, %19 ]
  %65 = phi <8 x i32> [ undef, %10 ], [ %57, %19 ]
  %66 = phi <8 x i32> [ undef, %10 ], [ %58, %19 ]
  %67 = phi i64 [ 0, %10 ], [ %59, %19 ]
  %68 = phi <8 x i32> [ zeroinitializer, %10 ], [ %55, %19 ]
  %69 = phi <8 x i32> [ zeroinitializer, %10 ], [ %56, %19 ]
  %70 = phi <8 x i32> [ zeroinitializer, %10 ], [ %57, %19 ]
  %71 = phi <8 x i32> [ zeroinitializer, %10 ], [ %58, %19 ]
  %72 = icmp eq i64 %15, 0
  br i1 %72, label %90, label %73

73:                                               ; preds = %62
  %74 = getelementptr inbounds i32, i32* %0, i64 %67
  %75 = getelementptr inbounds i32, i32* %74, i64 24
  %76 = bitcast i32* %75 to <8 x i32>*
  %77 = load <8 x i32>, <8 x i32>* %76, align 4, !tbaa !3
  %78 = add <8 x i32> %77, %71
  %79 = getelementptr inbounds i32, i32* %74, i64 16
  %80 = bitcast i32* %79 to <8 x i32>*
  %81 = load <8 x i32>, <8 x i32>* %80, align 4, !tbaa !3
  %82 = add <8 x i32> %81, %70
  %83 = getelementptr inbounds i32, i32* %74, i64 8
  %84 = bitcast i32* %83 to <8 x i32>*
  %85 = load <8 x i32>, <8 x i32>* %84, align 4, !tbaa !3
  %86 = add <8 x i32> %85, %69
  %87 = bitcast i32* %74 to <8 x i32>*
  %88 = load <8 x i32>, <8 x i32>* %87, align 4, !tbaa !3
  %89 = add <8 x i32> %88, %68
  br label %90

90:                                               ; preds = %62, %73
  %91 = phi <8 x i32> [ %63, %62 ], [ %89, %73 ]
  %92 = phi <8 x i32> [ %64, %62 ], [ %86, %73 ]
  %93 = phi <8 x i32> [ %65, %62 ], [ %82, %73 ]
  %94 = phi <8 x i32> [ %66, %62 ], [ %78, %73 ]
  %95 = add <8 x i32> %92, %91
  %96 = add <8 x i32> %93, %95
  %97 = add <8 x i32> %94, %96
  %98 = shufflevector <8 x i32> %97, <8 x i32> undef, <8 x i32> <i32 4, i32 5, i32 6, i32 7, i32 undef, i32 undef, i32 undef, i32 undef>
  %99 = add <8 x i32> %97, %98
  %100 = shufflevector <8 x i32> %99, <8 x i32> undef, <8 x i32> <i32 2, i32 3, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef>
  %101 = add <8 x i32> %99, %100
  %102 = shufflevector <8 x i32> %101, <8 x i32> undef, <8 x i32> <i32 1, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef>
  %103 = add <8 x i32> %101, %102
  %104 = extractelement <8 x i32> %103, i32 0
  %105 = icmp eq i64 %11, %5
  br i1 %105, label %106, label %7

106:                                              ; preds = %108, %90, %2
  %107 = phi i32 [ 0, %2 ], [ %104, %90 ], [ %113, %108 ]
  ret i32 %107

108:                                              ; preds = %7, %108
  %109 = phi i64 [ %114, %108 ], [ %8, %7 ]
  %110 = phi i32 [ %113, %108 ], [ %9, %7 ]
  %111 = getelementptr inbounds i32, i32* %0, i64 %109
  %112 = load i32, i32* %111, align 4, !tbaa !3
  %113 = add nsw i32 %112, %110
  %114 = add nuw nsw i64 %109, 1
  %115 = icmp eq i64 %114, %5
  br i1 %115, label %106, label %108, !llvm.loop !9
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: nounwind readnone uwtable
define dso_local i32 @main() local_unnamed_addr #2 {
  %1 = alloca [50 x i32], align 16
  %2 = bitcast [50 x i32]* %1 to i8*
  call void @llvm.lifetime.start.p0i8(i64 200, i8* nonnull %2) #3
  %3 = bitcast [50 x i32]* %1 to <8 x i32>*
  store <8 x i32> <i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8>, <8 x i32>* %3, align 16, !tbaa !3
  %4 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 8
  %5 = bitcast i32* %4 to <8 x i32>*
  store <8 x i32> <i32 9, i32 10, i32 11, i32 12, i32 13, i32 14, i32 15, i32 16>, <8 x i32>* %5, align 16, !tbaa !3
  %6 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 16
  %7 = bitcast i32* %6 to <8 x i32>*
  store <8 x i32> <i32 17, i32 18, i32 19, i32 20, i32 21, i32 22, i32 23, i32 24>, <8 x i32>* %7, align 16, !tbaa !3
  %8 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 24
  %9 = bitcast i32* %8 to <8 x i32>*
  store <8 x i32> <i32 25, i32 26, i32 27, i32 28, i32 29, i32 30, i32 31, i32 32>, <8 x i32>* %9, align 16, !tbaa !3
  %10 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 32
  %11 = bitcast i32* %10 to <8 x i32>*
  store <8 x i32> <i32 33, i32 34, i32 35, i32 36, i32 37, i32 38, i32 39, i32 40>, <8 x i32>* %11, align 16, !tbaa !3
  %12 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 40
  %13 = bitcast i32* %12 to <8 x i32>*
  store <8 x i32> <i32 41, i32 42, i32 43, i32 44, i32 45, i32 46, i32 47, i32 48>, <8 x i32>* %13, align 16, !tbaa !3
  %14 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 48
  store i32 49, i32* %14, align 16, !tbaa !3
  %15 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 49
  store i32 50, i32* %15, align 4, !tbaa !3
  %16 = bitcast [50 x i32]* %1 to <8 x i32>*
  %17 = load <8 x i32>, <8 x i32>* %16, align 16, !tbaa !3
  %18 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 8
  %19 = bitcast i32* %18 to <8 x i32>*
  %20 = load <8 x i32>, <8 x i32>* %19, align 16, !tbaa !3
  %21 = add <8 x i32> %20, %17
  %22 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 16
  %23 = bitcast i32* %22 to <8 x i32>*
  %24 = load <8 x i32>, <8 x i32>* %23, align 16, !tbaa !3
  %25 = add <8 x i32> %24, %21
  %26 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 24
  %27 = bitcast i32* %26 to <8 x i32>*
  %28 = load <8 x i32>, <8 x i32>* %27, align 16, !tbaa !3
  %29 = add <8 x i32> %28, %25
  %30 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 32
  %31 = bitcast i32* %30 to <8 x i32>*
  %32 = load <8 x i32>, <8 x i32>* %31, align 16, !tbaa !3
  %33 = add <8 x i32> %32, %29
  %34 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 40
  %35 = bitcast i32* %34 to <8 x i32>*
  %36 = load <8 x i32>, <8 x i32>* %35, align 16, !tbaa !3
  %37 = add <8 x i32> %36, %33
  %38 = shufflevector <8 x i32> %37, <8 x i32> undef, <8 x i32> <i32 4, i32 5, i32 6, i32 7, i32 undef, i32 undef, i32 undef, i32 undef>
  %39 = add <8 x i32> %37, %38
  %40 = shufflevector <8 x i32> %39, <8 x i32> undef, <8 x i32> <i32 2, i32 3, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef>
  %41 = add <8 x i32> %39, %40
  %42 = shufflevector <8 x i32> %41, <8 x i32> undef, <8 x i32> <i32 1, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef, i32 undef>
  %43 = add <8 x i32> %41, %42
  %44 = extractelement <8 x i32> %43, i32 0
  %45 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 48
  %46 = load i32, i32* %45, align 16, !tbaa !3
  %47 = add nsw i32 %46, %44
  %48 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 49
  %49 = load i32, i32* %48, align 4, !tbaa !3
  %50 = add nsw i32 %49, %47
  call void @llvm.lifetime.end.p0i8(i64 200, i8* nonnull %2) #3
  ret i32 %50
}

attributes #0 = { norecurse nounwind readonly uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+avx,+avx2,+cx8,+fxsr,+mmx,+popcnt,+sse,+sse2,+sse3,+sse4.1,+sse4.2,+ssse3,+x87,+xsave" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind willreturn }
attributes #2 = { nounwind readnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+avx,+avx2,+cx8,+fxsr,+mmx,+popcnt,+sse,+sse2,+sse3,+sse4.1,+sse4.2,+ssse3,+x87,+xsave" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 10.0.0 "}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
!7 = distinct !{!7, !8}
!8 = !{!"llvm.loop.isvectorized", i32 1}
!9 = distinct !{!9, !10, !8}
!10 = !{!"llvm.loop.unroll.runtime.disable"}
