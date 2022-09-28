i=IO.readlines('i').map &:to_i
p i.each_cons(2).count{_1<_2},i.each_cons(3).each_cons(2).count{_1.sum<_2.sum}
