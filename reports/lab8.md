# Lab 8 Report

## Implementation Summary

In this lab, I implemented `sys_get_time` and the deadlock detection switch. Before mutex and semaphore acquisition, the kernel maintains resource allocation, request, and available states, then uses a safety check to decide whether the request may cause deadlock. If a risk is detected, the request is rejected with an error code.