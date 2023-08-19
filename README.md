# Conformator










## Todo

- [ ] Dependency Tree Builder
- [ ] Function Builder
- [ ] Config Component
- [ ] Composer
- [ ] Runner


### Dependency Tree Builder

This is a parser which will try to construct dependency tree from the provided input file

e.g.
```
zoxide: cargo + fzf
cargo: curl
```

this should be parsed as:
```
                                                      
     +----------+                                     
     |  zoxide  |<------+                             
     +----------+       |                             
          ^             |                             
     +----+----+   +----+----+                        
     |  cargo  |   |   fzf   |                        
     +---------+   +---------+                        
          ^                                           
     +----+----+                                      
     |  curl   |                                      
     +---------+                                      

```


The dependency definitions should look like:

- main package
- dependencies with `+` as separator
- `;` for line completion
- `!` for indicating optional dependency
- `@` for indicating custom [builder](###Function Builder)


### Functions

This are simple units that will execute single argument instrustions

e.g.
```

bash "echo Hello World!";


```

example of functions are 

- formatted string
    "hello {}" value
- bash executor
    bash "echo Hello World!"

we can pass the result of one function to another using `$`

- `bash $ "echo Hello {}!" "world"`

> Function definition
>
> ```
> -- {name} does
> {expr};
> ```



### Processor Builder

This should be generic definition which expresses the behaviour of a dependency or installation

It should consist of:
- Steps on Installation
- Steps on Verification
- Steps on Uninstallation
- Steps on Configuration
- (Optional) Steps on Garbage Collection

### Config Component

This will be attached to the package, and will be executed after the dependency has been installed

### Composer - WIP

This should be a seamless way on composing the above 2 methods while also providing custom composation options for different package manager

