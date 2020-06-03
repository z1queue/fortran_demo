AFLAG = crus
SFlAG = -shared -o
FFLAGS = -O3 -Wall -fexceptions -g -Wno-uninitialized -c -fPIC  -static-libgfortran -static-libgcc -o
LBFGSB  = lbfgsb.f
LINPACK = linpack.f
BLAS    = blas.f
TIMER   = timer.f
STRING = string.f
HELLO = hello.f
static :
	@echo "static linking selected"
	@echo "Compiling"
	#gfortran $(LBFGSB) $(FFLAGS) $(OUTPUT)/lbfgsb.o
	#gfortran $(LINPACK) $(FFLAGS) $(OUTPUT)/linpack.o
	#gfortran $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	#gfortran $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	#gfortran $(STRING) $(FFLAGS) $(OUTPUT)/string.o
	gfortran -ffree-form  $(HELLO) $(FFLAGS) $(OUTPUT)/hello.o
	@echo "creating static library"
	$(AR) $(AFLAG) $(OUTPUT)/liblbfgs.a $(OUTPUT)/*.o
dylib :
	@echo "dynamic linking selected"
	@echo "Compiling"
	@echo $(OUTPUT)
	#gfortran $(LBFGSB) $(FFLAGS) $(OUTPUT)/lbfgsb.o
	#gfortran $(LINPACK) $(FFLAGS) $(OUTPUT)/linpack.o
	#gfortran $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	#gfortran $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	#gfortran $(STRING) $(FFLAGS) $(OUTPUT)/string.o
	gfortran -ffree-form  $(HELLO) $(FFLAGS) $(OUTPUT)/hello.o
	@echo "creating static library"
	gfortran $(SFlAG) $(OUTPUT)/liblbfgs.so $(OUTPUT)/*.o
