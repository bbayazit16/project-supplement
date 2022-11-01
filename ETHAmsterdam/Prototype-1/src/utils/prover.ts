import powmod = require("modular-power")
const pow = powmod.default

export default class CryptoBallot {
    private readonly P = 283
    private readonly Q = 47
    private readonly G = 60

    public readonly priv: number
    public readonly pub: number

    constructor(pub: number) {
        this.pub = pub
        // this.priv = Math.floor(Math.random() * this.Q)
        // this.pub = pow(this.G, this.priv, this.P)
    }

    public decrypt(A: number, B: number): number {
        const AI = pow(A, this.priv * (this.P - 2), this.P)
        const GM = (B * AI) % this.P

        let m = 0

        while (pow(this.G, m, this.P) != GM) {
            m++
        }

        return m
    }

    public sumC(votes: number[][]): [ number, number ] {
        let a = 1
        let b = 1

        for (const vote of votes) {
            a = (a * vote[0]) % this.P
            b = (a * vote[1]) % this.P
        }

        return [ a, b ]
    }

}

// /**
//  *
//  * @returns priv/pub
//  */
// export const keygen = (): [number, number] => {
//     const priv = Math.floor(Math.random() * Q)
//     const pub = pow(G, secret, P)

//     return [ priv, pub ]
// }

// /**
//  * Decrypt the sum of yes/no votes.
//  */
// export const decrypt = (priv: number, A: number, B: number): number => {
//     const AI = pow(A, priv * (P - 2), P)
//     const GM = (B * AI) % P

//     let m = 0

//     while (pow(G, m, P) != GM) {
//         m++
//     }

//     return m
// }

// /**
//  * Calculate the sum of yes/no votes.
//  */
// export const sumC = (votes: number[][]): [number, number] => {
//     let a = 1
//     let b = 1

//     votes.forEach(vote => {
//         a = (a * vote[0]) % P
//         b = (a * vote[1]) % P
//     })

//     return [ a, b ]
// }
