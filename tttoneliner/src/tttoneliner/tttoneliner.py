def main():B=111111111;{print(f'{B-111111111:09d}')for _ in range(5)if(B:=B+1*10**abs(9-int(input())))and(B:=B+2*10**abs(8-(B*1%9)))}
# def main():B=list('1'*9);{B.__setitem__(int(input())-1,'2')or B.__setitem__(int(''.join(B))*1%9,'3')or print(int(''.join(B))-111111111)for _ in range(5)}