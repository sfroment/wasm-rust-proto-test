"use client";

import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { SayHelloRequestWrapper, type GreeterClient } from "proto-client";
import { useForm } from "react-hook-form";
import { GreeterServicePromiseClient } from "@/gen/helloworld/v1/helloworld_grpc_web_pb";
import { SayHelloRequest } from "@/gen/helloworld/v1/helloworld_pb";

export function Test() {
  const client = new GreeterServicePromiseClient("http://localhost:50051");
  const form = useForm();

  const onSubmit = (values) => {
    //console.log("Submitting:", values);
    const request = new SayHelloRequest();
    request.setName(values.username);
    client.sayHello(request).then((response) => {
      console.log("Response received:", response.getMessage());
    });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="username"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input placeholder="shadcn" {...field} />
              </FormControl>
              <FormDescription>
                This is your public display name.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
