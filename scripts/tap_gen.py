import random
import math

# lookup table for the first 256 terms in the sequence "sum of primes up to n"
SUM_P_N = [2, 5, 10, 17, 28, 41, 58, 77, 100, 129, 160, 197, 238, 281, 328, 381, 
440, 501, 568, 639, 712, 791, 874, 963, 1060, 1161, 1264, 1371, 1480, 1593, 1720, 
1851, 1988, 2127, 2276, 2427, 2584, 2747, 2914, 3087, 3266, 3447, 3638, 3831, 4028, 
4227, 4438, 4661, 4888, 5117, 5350, 5589, 5830, 6081, 6338, 6601, 6870, 7141, 7418, 
7699, 7982, 8275, 8582, 8893, 9206, 9523, 9854, 10191, 10538, 10887, 11240, 11599, 
11966, 12339, 12718, 13101, 13490, 13887, 14288, 14697, 15116, 15537, 15968, 16401, 
16840, 17283, 17732, 18189, 18650, 19113, 19580, 20059, 20546, 21037, 21536, 22039, 
22548, 23069, 23592, 24133, 24680, 25237, 25800, 26369, 26940, 27517, 28104, 28697, 
29296, 29897, 30504, 31117, 31734, 32353, 32984, 33625, 34268, 34915, 35568, 36227, 
36888, 37561, 38238, 38921, 39612, 40313, 41022, 41741, 42468, 43201, 43940, 44683, 
45434, 46191, 46952, 47721, 48494, 49281, 50078, 50887, 51698, 52519, 53342, 54169, 
54998, 55837, 56690, 57547, 58406, 59269, 60146, 61027, 61910, 62797, 63704, 64615, 
65534, 66463, 67400, 68341, 69288, 70241, 71208, 72179, 73156, 74139, 75130, 76127, 
77136, 78149, 79168, 80189, 81220, 82253, 83292, 84341, 85392, 86453, 87516, 88585, 
89672, 90763, 91856, 92953, 94056, 95165, 96282, 97405, 98534, 99685, 100838, 102001, 
103172, 104353, 105540, 106733, 107934, 109147, 110364, 111587, 112816, 114047, 115284, 
116533, 117792, 119069, 120348, 121631, 122920, 124211, 125508, 126809, 128112, 129419, 
130738, 132059, 133386, 134747, 136114, 137487, 138868, 140267, 141676, 143099, 144526, 
145955, 147388, 148827, 150274, 151725, 153178, 154637, 156108, 157589, 159072, 160559, 
162048, 163541, 165040, 166551, 168074, 169605, 171148, 172697, 174250, 175809, 177376, 
178947, 180526, 182109, 183706, 185307, 186914, 188523, 190136, 191755]

# lookup table for the first 256 primes
PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 
71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 
163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 
251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 
349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 
443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 
557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 
647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 
757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 
863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 
983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 
1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 
1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 
1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 
1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 
1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 
1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613]

def clipped_random() -> float:
    return math.tanh(random.uniform(-3.0, 3.0)) / math.tanh(3.0)

def sum_less_than(max: int) -> int:
    for pair in zip(SUM_P_N, range(len(SUM_P_N))):
        if pair[0] > max:
            return pair[1]
    return -1

def unity_gain_coeffs(size: int):
    coeffs = []
    if size % 2 == 1:
        for i in range(size//2):
            tap = clipped_random()
            coeffs.append(tap)
            coeffs.append(-tap)
        coeffs.append(1.0)

    else:
        for i in range(size//2 - 1):
            tap = clipped_random()
            coeffs.append(tap)
            coeffs.append(-tap)
        coeffs.append(0.5)
        coeffs.append(0.5)
    
    return random.sample(coeffs, size)





if __name__ == "__main__":
    dense_coeffs = unity_gain_coeffs(1028)
    sparse_coeffs = unity_gain_coeffs(290)
    print(f"pub const DENSE_COEFFS: [usize; 1028] = {dense_coeffs};")
    print()
    print(f"pub const SPARSE_COEFFS: [usize; 290] = {sparse_coeffs};")
    print()
    print()
    
    print("// SPARSE MULTICHANNEL:")
    max_idx = sum_less_than(2**14)
    pool = PRIMES[:max_idx]
    sparse_a_pool = random.sample(pool, max_idx)
    sparse_b_pool = random.sample(pool, max_idx)
    sparse_c_pool = random.sample(pool, max_idx)
    sparse_d_pool = random.sample(pool, max_idx)
    sparse_e_pool = random.sample(pool, max_idx)
    sparse_a = []
    sparse_a_accum = 0
    for val in sparse_a_pool:
        sparse_a_accum += val
        sparse_a.append(sparse_a_accum)
    sparse_b = []
    sparse_b_accum = 0
    for val in sparse_b_pool:
        sparse_b_accum += val
        sparse_b.append(sparse_b_accum)
    sparse_c = []
    sparse_c_accum = 0
    for val in sparse_c_pool:
        sparse_c_accum += val
        sparse_c.append(sparse_c_accum)
    sparse_d = []
    sparse_d_accum = 0
    for val in sparse_d_pool:
        sparse_d_accum += val
        sparse_d.append(sparse_d_accum)
    sparse_e = []
    sparse_e_accum = 0
    for val in sparse_e_pool:
        sparse_e_accum += val
        sparse_e.append(sparse_e_accum)
    print(f"pub const SPARSE_A: [usize; {max_idx}] = {sparse_a};")
    print()
    print(f"pub const SPARSE_B: [usize; {max_idx}] = {sparse_b};")
    print()
    print(f"pub const SPARSE_C: [usize; {max_idx}] = {sparse_c};")
    print()
    print(f"pub const SPARSE_D: [usize; {max_idx}] = {sparse_d};")
    print()
    print(f"pub const SPARSE_E: [usize; {max_idx}] = {sparse_e};")

    sparse_a_coeffs = unity_gain_coeffs(max_idx)
    sparse_b_coeffs = unity_gain_coeffs(max_idx)
    sparse_c_coeffs = unity_gain_coeffs(max_idx)
    sparse_d_coeffs = unity_gain_coeffs(max_idx)
    sparse_e_coeffs = unity_gain_coeffs(max_idx)

    print(f"pub const SPARSE_A_COEFFS: [usize; {max_idx}] = {sparse_a_coeffs};")
    print()
    print(f"pub const SPARSE_B_COEFFS: [usize; {max_idx}] = {sparse_b_coeffs};")
    print()
    print(f"pub const SPARSE_C_COEFFS: [usize; {max_idx}] = {sparse_c_coeffs};")
    print()
    print(f"pub const SPARSE_D_COEFFS: [usize; {max_idx}] = {sparse_d_coeffs};")
    print()
    print(f"pub const SPARSE_E_COEFFS: [usize; {max_idx}] = {sparse_e_coeffs};")