use itertools::Itertools;

const MAGIC_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
const CHUNK_LENGTH: usize = 16;

pub fn hash(plaintext: &str) -> Vec<u8> {
    let instructions = plaintext.trim().chars().map(|x| x as u8).chain(MAGIC_SUFFIX).collect_vec();
    let hash = twist(&instructions, 64);
    xor_shrink(&hash)
}

pub fn twist(instructions: &[u8], rounds: usize) -> Vec<u8> {
    let mut list = (u8::MIN..=u8::MAX).into_iter().collect_vec();

    let mut cursor = 0usize;
    let mut skip_size = 0usize;

    for _ in 0..rounds {
        for pos in instructions {
            for i in 0..*pos {
                let left = cursor + i as usize;
                let right = cursor + *pos as usize - i as usize - 1;
                if left >= right {
                    break;
                }
                let list_len = list.len();
                list.swap(left % list_len, right % list_len);
            }
            cursor += *pos as usize + skip_size;
            cursor %= list.len();
            skip_size += 1;
            skip_size %= list.len();
        }
    }

    list
}

fn xor_shrink(input: &[u8]) -> Vec<u8> {
    assert!(input.len() % CHUNK_LENGTH == 0);

    input.iter()
        .chunks(CHUNK_LENGTH)
        .into_iter()
        .map(|x: itertools::Chunk<'_, std::slice::Iter<'_, u8>>| {
            x.into_iter().cloned().reduce(|acc, e| acc ^ e).unwrap()
        })
        .collect_vec()
}

pub fn bytes_to_hex(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len() * 2);

    input.into_iter().for_each(|x| {
        out.push_str(&format!("{x:02x}"));
    });

    out
}