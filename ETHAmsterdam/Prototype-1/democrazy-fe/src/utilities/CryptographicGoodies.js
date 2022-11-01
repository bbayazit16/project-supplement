import {BigInteger} from "jsbn";

// PUBLIC PARAMETERS
const p = new BigInteger("142950481577612897377251366207350601085193026787763232208511322259955075211826388565191137969675785957228922178875744018870301928203434246727650650452188476517559379655516949015006180412307375960073546778478575555767086902406147563214485604901264760329721957156402926704404814419844454185694597535438114709207");
const q = new BigInteger("71475240788806448688625683103675300542596513393881616104255661129977537605913194282595568984837892978614461089437872009435150964101717123363825325226094238258779689827758474507503090206153687980036773389239287777883543451203073781607242802450632380164860978578201463352202407209922227092847298767719057354057");
const g = new BigInteger("5");

// PRIVATE PARAMETERS
const secret_key = new BigInteger("2380831451759006579882553120960984113277981577916531573623778541891671800956226624640322916414543030985934711395325938960305715079214976582052109890021687068");

function random(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

export function generate_pk(sk) {
  return g.modPow(new BigInteger(sk), p);
}

export function customHash(values) {
  let h = new BigInteger("0");
  for (let i = 0; i < values.length; i++) {
    h = (  h.add(new BigInteger("10").pow(i)).multiply(values[i]).mod(q) ).mod(q);
    //    h = (h + Math.pow(10, i) * values[i] % q) % q;
  }
  return h;
}

export function validVoteProof(pk, v, a, b, r) {
  let a0, a1, b0, b1, c0, c1, r0, r1;
  let c;

  if (v === 0) {
    c1 = new BigInteger(random(0, q - 1).toString());
    r0 = new BigInteger(random(0, q - 1).toString());
    r1 = new BigInteger(random(0, q - 1).toString());

    a1 = g.modPow(r1, p).multiply(new BigInteger(a).modPow(new BigInteger(c1).multiply(p.subtract(2)), p));//(pow(g, r1, p) * pow(a, c1 * (p - 2), p)) % p;
    b1 = (pk.modPow(r1, p)).multiply(b.pow(g, p.subtract(2), p).mod(p).modPow(c1.multiply(p.subtract(2), p)));
    a0 = g.modPow(r0, p);
    b0 = pk.modPow(r0, p);
    c = customHash([pk, a, b, a0, b0, a1, b1]);
    c0 = c1.subtract(c).abs();

    c0 = q.add(c1.subtract(c).mod(q)).mod(q);

    r0 = r0.add(c0.multiply(r).mod(q)).mod(q);

    return [a0, a1, b0, b1, c0, c1, r0, r1];
  } else if (v === 1) {
    c0 = new BigInteger(random(0, q - 1).toString());
    r0 = new BigInteger(random(0, q - 1).toString());
    r1 = new BigInteger(random(0, q - 1).toString());
    a0 = g.modPow(r0, p).multiply(a.modPow(c0.multiply(p.subtract(2))), p).mod(p);
    b0 = pk.modPow(r0, p).multiply(b.modPow(c0.multiply(p.subtract(2))), p).mod(p);
    a1 = g.modPow(r1, p);
    b1 = pk.modPow(r1, p);
    c = customHash([pk, a, b, a0, b0, a1, b1]);
    c1 = c0.subtract(c).abs()
    c1 = q.add(c0.subtract(c).mod(q)).mod(q);
    r1 = r1.add(c1.multiply(r).mod(q)).mod(q);
    return [a0, a1, b0, b1, c0, c1, r0, r1];
  } else {
    return [0, 0, 0, 0, 0, 0, 0, 0];
  }
}
