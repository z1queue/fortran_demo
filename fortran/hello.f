SUBROUTINE hello
    ix = 1
END


SUBROUTINE add(a , b , sum)
    IMPLICIT NONE
    integer , INTENT(IN)::a    
    integer , INTENT(IN)::b
    integer , INTENT(OUT)::sum
    integer::temp

    WRITE (*,*) a
    WRITE (*,*) b
   
    sum = a + b
    WRITE (*,*) sum
END SUBROUTINE add
