use ark_serialize::CanonicalDeserialize;

use crate::inner_product_argument::{CommitKey, Instance, Proof};
const COMMIT_KEY_BYTES: &'static str = "CAAAAAAAAACdCH9GccfV39AeGVy7HfoafpnPycDG3mTfZFrAP4yLmV58MOMpVnnd+N/PQPJZB7MEDQLFQM0mcBcX6KyR1JLLuqxH0weTEG9pdjkR9gXJkFHywrzbidCFG/X4jmnp9fBS1Dq59AgMgLJx5XtYC5+2SkZRUL9RAKRe6I/+1gXwv3nAJzmDnbOB7fYVjQE6rP0o8+ug0KE8DsZz+wAPWHRRQrfW96sx91gNz2IQarA5OSsqN3mBZNnaDbzibTohsGVHY6qTWg1sZCvnEK+ydGfwAHcMby25TrWvWB1zcUlkUL+bb36OncTqQY92Kkky23kVjMDySDMuQQB907aMVoExMnuluWWJVbupS80Dqyp1CA+K+lguoj+okO0b6ODYLro=";

const INSTANCE_AND_PROOF_BYTES: [&'static str; 2] = [
    "ir8oQmlflodJky2AD9aoBlwPGPdvUIn76Qq7T+Bx4moIAAAAAAAAAM05+NtcfZ5IO5frFUEB4jN83JcTntCIvN6qNKVmDhgI5mKxry/cBj7PjshABNymrSdOv/XuzJGRrASsL4L7agPyNINAYPQt9Yd74CjGKz/rmptvhyEsHgQqn/fcI/7nDX5F17tLi7z0VtrJGOC6e24YUZ+MKFnt9xdX2uPhu5EIll4An28ygHbU24Qc1RX6mFr+axeFFivQo1BjQUwf2AU8Rakq8O/NPW28bo1As+wp3nVFJLv/o/MkilBnhuXVBsu9HAesl9gHoGXttu2Mf46ipsS0UUBi7lPTa3ymR7wGWjEkxSZ6E3zXs/f+79KT2UiUwbdbjlYJI28LXtPZ9gwo7Bid8PoEi93gobn1E3Qlg6Y0UUjHwuTMUrrjSTgQVOERjASsf9LDNgeZr0dc6gi27mBt9THa1VuCjf6pn9yK9SAqfLZrh1nksjUQZP5E2c3T4SP4kf0SUmuEuh9r/ZkIAAAAAAAAAJuiWXTvJTaJquMJ6UcoLvOp8S2X+cyPWkOheOlynr8DL6VuxUV25oVGAW+uu3lLN6Efn1Gd6SOgKLckWIlM1AcCjd8gv2TlIlP5I+KtJ1yQL2sbuw7wPdHuKpVDOxorBejjSVaH8RRVMI9/7rJuNmuaewLrG/Aq4FEwzXe/y38KLe6P3tu14ntXc1v4VJg6a9wS4w8cXPq5zG5ZL1pEQwAWl2Xw8j+6RtHjchFLmAv1Pd0EcxFhtgBNNHNpdqhJAg4Jx6mjh3wHG4lZkFDkl9Q1bVtbZAVh8iSoRXwXOXsI5H6Kx0ipSSR9WsYnjFNGrWgEPZNmYe/uf4C3NKXNxwOOE+dYFeTGUUrj2PJCL4yXJQtaOVT+HDo0xtF8dA68AQoNkDEWyt1ncUbuelaCACl0VGcoiMbI1qylWDE76mQB",
    "ir8oQmlflodJky2AD9aoBlwPGPdvUIn76Qq7T+Bx4moIAAAAAAAAAM05+NtcfZ5IO5frFUEB4jN83JcTntCIvN6qNKVmDhgI5mKxry/cBj7PjshABNymrSdOv/XuzJGRrASsL4L7agPyNINAYPQt9Yd74CjGKz/rmptvhyEsHgQqn/fcI/7nDX5F17tLi7z0VtrJGOC6e24YUZ+MKFnt9xdX2uPhu5EIll4An28ygHbU24Qc1RX6mFr+axeFFivQo1BjQUwf2AU8Rakq8O/NPW28bo1As+wp3nVFJLv/o/MkilBnhuXVBsu9HAesl9gHoGXttu2Mf46ipsS0UUBi7lPTa3ymR7wGWjEkxSZ6E3zXs/f+79KT2UiUwbdbjlYJI28LXtPZ9gwgzQ9q7nJCNjnM0ZihHb8osXE+lY9ZLgg2UK/ckY4JEIy2Bt+IzqiwnrDmnkOhq0/KfXXOZzWkMQTwLXyClWoviyAcIijtc6hF9+3HAl2dNb/37rWE106q/6uK6LbhDREIAAAAAAAAAP1RSBw3PvBPG4ntyo4R18++OpEOrzIQVtEjGpzECXUJEppprdaJdzlAiopoyJOpzfmRkzYtZjK0/+QOLctEDQMx1O7++cSsayZt/TAp3SDLhPN1G74i/5ljYTIR9XCCArZaVA4JVusXhwg9mkeYyGmlNd2zxLMdeSkVT28Yh7MFcFSm6XP9MVtlx5zV/aiM7uJn4OgPGBkK6ZQXFUC2/QAQ54E08vZXo1M9PkXv3qaO/BmWL6qXnUAT5u2znrI5CdZ01rpJFaTHVJ12EoeqYF1sVbdOsFnIo5WsGqei1xwFpOJCvaTV5iNA/HUT3/BlI88dOeOOhb0Jn/f6zKZungLus1pim4mqCAVm1DSVhWZ77Hfv1LC8ii2TbZ6v2AQhBawub5eLCD8zb9Dfh6s93qNlHdNQ+eONQOne0Q5u+EII"
];

pub fn puzzle_data() -> (CommitKey, [(Instance, Proof); 2]) {
    let ck = CommitKey::deserialize(&*base64::decode(COMMIT_KEY_BYTES).unwrap()).unwrap();
    let instance_proof_1: (Instance, Proof) =
        CanonicalDeserialize::deserialize(&*base64::decode(INSTANCE_AND_PROOF_BYTES[0]).unwrap())
            .unwrap();
    let instance_proof_2: (Instance, Proof) =
        CanonicalDeserialize::deserialize(&*base64::decode(INSTANCE_AND_PROOF_BYTES[1]).unwrap())
            .unwrap();
    (ck, [instance_proof_1, instance_proof_2])
}
