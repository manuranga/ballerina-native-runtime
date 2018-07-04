# ballerina-native-runtime

## Gen IR from ballerina
1. Checkout https://github.com/manuranga/ballerina/tree/native branch in your ballerina-lang repo
2. (can this step if you have build recently)
   - switch to previous commit `git checkout HEAD~1`
   - maven build `mvn clean install -o -Dmaven.test.skip=true -e -Dcheckstyle.skip=true -Dspotbugs.skip=true -Dfindbugs.skip=true -P ballerina -am -pl :ballerina-cli-utils` 
   - switch back to native branch
    
3. `mkdir tmp/hello/` and put `hello.bal` in it with https://ballerina.io/learn/by-example/hello-world.html
4. open 'GenerateBalx.java' form the IDE and run
5. you should get `/tmp/hello/hello.bc`

## Link to native
1. Run `cargo build --release` form `<ballerina-native-runtime-checkout-dir>`
2. Go to `/tmp/hello/` and run `llc-6.0 -filetype=obj hello.bc` should gen `hello.o`
3. Run `gcc hello.o -L <ballerina-native-runtime-checkout-dir>/target/release -lballerinart`
4. Run `export LD_LIBRARY_PATH=/home/manu/checkouts/ballerina-native-runtime/target/release`
5. Run `./a.out`
