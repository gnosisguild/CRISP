pragma circom 2.0.0;

template Vote() {
    // private
    signal input vote;
    vote === 0 || 1;
}

component main = Vote();