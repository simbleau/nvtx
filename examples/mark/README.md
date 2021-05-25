# Using marks

Marks are used to describe events at a specific time during the execution of a respective application.

They are visible in the events of the NVTX trace in programs such as NVIDIA NSight Systems.

![Screenshot from 2021-05-24 19-53-17](https://user-images.githubusercontent.com/48108917/119422310-d3585b00-bcce-11eb-976c-7c0771ab3da1.png)


# Running the example (NVIDIA NSight Systems)

 1. Open NVIDIA NSight Systems
 2. Start a new project
 3. Select a target host for profiling (local computer is fine)
 4. Set the target application information to:
    * Command line with arguments: `cargo run --example mark`
    * Set the working directory to the nvtx-rs directory path
 5. Collect NVTX trace
 6. Start the project (wait for completion)
 7. Open the report
 8. Right click the NVTX trace layer and "Show in Events View"
