; ModuleID = 'simd.c'
source_filename = "simd.c"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.26.28806"

; Function Attrs: norecurse nounwind readonly uwtable
define dso_local i32 @sumint(i32* nocapture readonly %0, i32 %1) local_unnamed_addr #0 {
  %3 = icmp sgt i32 %1, 0
  br i1 %3, label %4, label %95

4:                                                ; preds = %2
  %5 = zext i32 %1 to i64
  %6 = icmp ult i32 %1, 8
  br i1 %6, label %7, label %10

7:                                                ; preds = %85, %4
  %8 = phi i64 [ 0, %4 ], [ %11, %85 ]
  %9 = phi i32 [ 0, %4 ], [ %93, %85 ]
  br label %97

10:                                               ; preds = %4
  %11 = and i64 %5, 4294967288
  %12 = add nsw i64 %11, -8
  %13 = lshr exact i64 %12, 3
  %14 = add nuw nsw i64 %13, 1
  %15 = and i64 %14, 3
  %16 = icmp ult i64 %12, 24
  br i1 %16, label %62, label %17

17:                                               ; preds = %10
  %18 = sub nsw i64 %14, %15
  br label %19

19:                                               ; preds = %19, %17
  %20 = phi i64 [ 0, %17 ], [ %59, %19 ]
  %21 = phi <4 x i32> [ zeroinitializer, %17 ], [ %57, %19 ]
  %22 = phi <4 x i32> [ zeroinitializer, %17 ], [ %58, %19 ]
  %23 = phi i64 [ %18, %17 ], [ %60, %19 ]
  %24 = getelementptr inbounds i32, i32* %0, i64 %20
  %25 = bitcast i32* %24 to <4 x i32>*
  %26 = load <4 x i32>, <4 x i32>* %25, align 4, !tbaa !3
  %27 = getelementptr inbounds i32, i32* %24, i64 4
  %28 = bitcast i32* %27 to <4 x i32>*
  %29 = load <4 x i32>, <4 x i32>* %28, align 4, !tbaa !3
  %30 = add <4 x i32> %26, %21
  %31 = add <4 x i32> %29, %22
  %32 = or i64 %20, 8
  %33 = getelementptr inbounds i32, i32* %0, i64 %32
  %34 = bitcast i32* %33 to <4 x i32>*
  %35 = load <4 x i32>, <4 x i32>* %34, align 4, !tbaa !3
  %36 = getelementptr inbounds i32, i32* %33, i64 4
  %37 = bitcast i32* %36 to <4 x i32>*
  %38 = load <4 x i32>, <4 x i32>* %37, align 4, !tbaa !3
  %39 = add <4 x i32> %35, %30
  %40 = add <4 x i32> %38, %31
  %41 = or i64 %20, 16
  %42 = getelementptr inbounds i32, i32* %0, i64 %41
  %43 = bitcast i32* %42 to <4 x i32>*
  %44 = load <4 x i32>, <4 x i32>* %43, align 4, !tbaa !3
  %45 = getelementptr inbounds i32, i32* %42, i64 4
  %46 = bitcast i32* %45 to <4 x i32>*
  %47 = load <4 x i32>, <4 x i32>* %46, align 4, !tbaa !3
  %48 = add <4 x i32> %44, %39
  %49 = add <4 x i32> %47, %40
  %50 = or i64 %20, 24
  %51 = getelementptr inbounds i32, i32* %0, i64 %50
  %52 = bitcast i32* %51 to <4 x i32>*
  %53 = load <4 x i32>, <4 x i32>* %52, align 4, !tbaa !3
  %54 = getelementptr inbounds i32, i32* %51, i64 4
  %55 = bitcast i32* %54 to <4 x i32>*
  %56 = load <4 x i32>, <4 x i32>* %55, align 4, !tbaa !3
  %57 = add <4 x i32> %53, %48
  %58 = add <4 x i32> %56, %49
  %59 = add i64 %20, 32
  %60 = add i64 %23, -4
  %61 = icmp eq i64 %60, 0
  br i1 %61, label %62, label %19, !llvm.loop !7

62:                                               ; preds = %19, %10
  %63 = phi <4 x i32> [ undef, %10 ], [ %57, %19 ]
  %64 = phi <4 x i32> [ undef, %10 ], [ %58, %19 ]
  %65 = phi i64 [ 0, %10 ], [ %59, %19 ]
  %66 = phi <4 x i32> [ zeroinitializer, %10 ], [ %57, %19 ]
  %67 = phi <4 x i32> [ zeroinitializer, %10 ], [ %58, %19 ]
  %68 = icmp eq i64 %15, 0
  br i1 %68, label %85, label %69

69:                                               ; preds = %62, %69
  %70 = phi i64 [ %82, %69 ], [ %65, %62 ]
  %71 = phi <4 x i32> [ %80, %69 ], [ %66, %62 ]
  %72 = phi <4 x i32> [ %81, %69 ], [ %67, %62 ]
  %73 = phi i64 [ %83, %69 ], [ %15, %62 ]
  %74 = getelementptr inbounds i32, i32* %0, i64 %70
  %75 = bitcast i32* %74 to <4 x i32>*
  %76 = load <4 x i32>, <4 x i32>* %75, align 4, !tbaa !3
  %77 = getelementptr inbounds i32, i32* %74, i64 4
  %78 = bitcast i32* %77 to <4 x i32>*
  %79 = load <4 x i32>, <4 x i32>* %78, align 4, !tbaa !3
  %80 = add <4 x i32> %76, %71
  %81 = add <4 x i32> %79, %72
  %82 = add i64 %70, 8
  %83 = add i64 %73, -1
  %84 = icmp eq i64 %83, 0
  br i1 %84, label %85, label %69, !llvm.loop !9

85:                                               ; preds = %69, %62
  %86 = phi <4 x i32> [ %63, %62 ], [ %80, %69 ]
  %87 = phi <4 x i32> [ %64, %62 ], [ %81, %69 ]
  %88 = add <4 x i32> %87, %86
  %89 = shufflevector <4 x i32> %88, <4 x i32> undef, <4 x i32> <i32 2, i32 3, i32 undef, i32 undef>
  %90 = add <4 x i32> %88, %89
  %91 = shufflevector <4 x i32> %90, <4 x i32> undef, <4 x i32> <i32 1, i32 undef, i32 undef, i32 undef>
  %92 = add <4 x i32> %90, %91
  %93 = extractelement <4 x i32> %92, i32 0
  %94 = icmp eq i64 %11, %5
  br i1 %94, label %95, label %7

95:                                               ; preds = %97, %85, %2
  %96 = phi i32 [ 0, %2 ], [ %93, %85 ], [ %102, %97 ]
  ret i32 %96

97:                                               ; preds = %7, %97
  %98 = phi i64 [ %103, %97 ], [ %8, %7 ]
  %99 = phi i32 [ %102, %97 ], [ %9, %7 ]
  %100 = getelementptr inbounds i32, i32* %0, i64 %98
  %101 = load i32, i32* %100, align 4, !tbaa !3
  %102 = add nsw i32 %101, %99
  %103 = add nuw nsw i64 %98, 1
  %104 = icmp eq i64 %103, %5
  br i1 %104, label %95, label %97, !llvm.loop !11
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
  %3 = bitcast [50 x i32]* %1 to <4 x i32>*
  store <4 x i32> <i32 1, i32 2, i32 3, i32 4>, <4 x i32>* %3, align 16, !tbaa !3
  %4 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 4
  %5 = bitcast i32* %4 to <4 x i32>*
  store <4 x i32> <i32 5, i32 6, i32 7, i32 8>, <4 x i32>* %5, align 16, !tbaa !3
  %6 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 8
  %7 = bitcast i32* %6 to <4 x i32>*
  store <4 x i32> <i32 9, i32 10, i32 11, i32 12>, <4 x i32>* %7, align 16, !tbaa !3
  %8 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 12
  %9 = bitcast i32* %8 to <4 x i32>*
  store <4 x i32> <i32 13, i32 14, i32 15, i32 16>, <4 x i32>* %9, align 16, !tbaa !3
  %10 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 16
  %11 = bitcast i32* %10 to <4 x i32>*
  store <4 x i32> <i32 17, i32 18, i32 19, i32 20>, <4 x i32>* %11, align 16, !tbaa !3
  %12 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 20
  %13 = bitcast i32* %12 to <4 x i32>*
  store <4 x i32> <i32 21, i32 22, i32 23, i32 24>, <4 x i32>* %13, align 16, !tbaa !3
  %14 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 24
  %15 = bitcast i32* %14 to <4 x i32>*
  store <4 x i32> <i32 25, i32 26, i32 27, i32 28>, <4 x i32>* %15, align 16, !tbaa !3
  %16 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 28
  %17 = bitcast i32* %16 to <4 x i32>*
  store <4 x i32> <i32 29, i32 30, i32 31, i32 32>, <4 x i32>* %17, align 16, !tbaa !3
  %18 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 32
  %19 = bitcast i32* %18 to <4 x i32>*
  store <4 x i32> <i32 33, i32 34, i32 35, i32 36>, <4 x i32>* %19, align 16, !tbaa !3
  %20 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 36
  %21 = bitcast i32* %20 to <4 x i32>*
  store <4 x i32> <i32 37, i32 38, i32 39, i32 40>, <4 x i32>* %21, align 16, !tbaa !3
  %22 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 40
  %23 = bitcast i32* %22 to <4 x i32>*
  store <4 x i32> <i32 41, i32 42, i32 43, i32 44>, <4 x i32>* %23, align 16, !tbaa !3
  %24 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 44
  %25 = bitcast i32* %24 to <4 x i32>*
  store <4 x i32> <i32 45, i32 46, i32 47, i32 48>, <4 x i32>* %25, align 16, !tbaa !3
  %26 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 48
  store i32 49, i32* %26, align 16, !tbaa !3
  %27 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 49
  store i32 50, i32* %27, align 4, !tbaa !3
  %28 = bitcast [50 x i32]* %1 to <4 x i32>*
  %29 = load <4 x i32>, <4 x i32>* %28, align 16, !tbaa !3
  %30 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 4
  %31 = bitcast i32* %30 to <4 x i32>*
  %32 = load <4 x i32>, <4 x i32>* %31, align 16, !tbaa !3
  %33 = add <4 x i32> %32, %29
  %34 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 8
  %35 = bitcast i32* %34 to <4 x i32>*
  %36 = load <4 x i32>, <4 x i32>* %35, align 16, !tbaa !3
  %37 = add <4 x i32> %36, %33
  %38 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 12
  %39 = bitcast i32* %38 to <4 x i32>*
  %40 = load <4 x i32>, <4 x i32>* %39, align 16, !tbaa !3
  %41 = add <4 x i32> %40, %37
  %42 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 16
  %43 = bitcast i32* %42 to <4 x i32>*
  %44 = load <4 x i32>, <4 x i32>* %43, align 16, !tbaa !3
  %45 = add <4 x i32> %44, %41
  %46 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 20
  %47 = bitcast i32* %46 to <4 x i32>*
  %48 = load <4 x i32>, <4 x i32>* %47, align 16, !tbaa !3
  %49 = add <4 x i32> %48, %45
  %50 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 24
  %51 = bitcast i32* %50 to <4 x i32>*
  %52 = load <4 x i32>, <4 x i32>* %51, align 16, !tbaa !3
  %53 = add <4 x i32> %52, %49
  %54 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 28
  %55 = bitcast i32* %54 to <4 x i32>*
  %56 = load <4 x i32>, <4 x i32>* %55, align 16, !tbaa !3
  %57 = add <4 x i32> %56, %53
  %58 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 32
  %59 = bitcast i32* %58 to <4 x i32>*
  %60 = load <4 x i32>, <4 x i32>* %59, align 16, !tbaa !3
  %61 = add <4 x i32> %60, %57
  %62 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 36
  %63 = bitcast i32* %62 to <4 x i32>*
  %64 = load <4 x i32>, <4 x i32>* %63, align 16, !tbaa !3
  %65 = add <4 x i32> %64, %61
  %66 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 40
  %67 = bitcast i32* %66 to <4 x i32>*
  %68 = load <4 x i32>, <4 x i32>* %67, align 16, !tbaa !3
  %69 = add <4 x i32> %68, %65
  %70 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 44
  %71 = bitcast i32* %70 to <4 x i32>*
  %72 = load <4 x i32>, <4 x i32>* %71, align 16, !tbaa !3
  %73 = add <4 x i32> %72, %69
  %74 = shufflevector <4 x i32> %73, <4 x i32> undef, <4 x i32> <i32 2, i32 3, i32 undef, i32 undef>
  %75 = add <4 x i32> %73, %74
  %76 = shufflevector <4 x i32> %75, <4 x i32> undef, <4 x i32> <i32 1, i32 undef, i32 undef, i32 undef>
  %77 = add <4 x i32> %75, %76
  %78 = extractelement <4 x i32> %77, i32 0
  %79 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 48
  %80 = load i32, i32* %79, align 16, !tbaa !3
  %81 = add nsw i32 %80, %78
  %82 = getelementptr inbounds [50 x i32], [50 x i32]* %1, i64 0, i64 49
  %83 = load i32, i32* %82, align 4, !tbaa !3
  %84 = add nsw i32 %83, %81
  call void @llvm.lifetime.end.p0i8(i64 200, i8* nonnull %2) #3
  ret i32 %84
}

attributes #0 = { norecurse nounwind readonly uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind willreturn }
attributes #2 = { nounwind readnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
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
!9 = distinct !{!9, !10}
!10 = !{!"llvm.loop.unroll.disable"}
!11 = distinct !{!11, !12, !8}
!12 = !{!"llvm.loop.unroll.runtime.disable"}
