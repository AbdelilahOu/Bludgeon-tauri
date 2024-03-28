<script setup lang="ts">
import { SheetContent } from "@/components/ui/sheet";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Mail, User, Phone, MapPin } from "lucide-vue-next";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
</script>

<template>
  <SheetContent
    @close-auto-focus="(e) => e.preventDefault()"
    class="w-1/2 min-w-[540px] p-0"
  >
    <ScrollArea class="h-screen">
      <div class="w-full h-fit flex px-2">
        <div class="w-full h-48 flex items-center justify-center border-b">
          <Avatar size="lg">
            <AvatarImage
              :src="($route.query.image as string) ?? ''"
              alt="@shadcn"
            />
            <AvatarFallback class="text-sm">
              {{ $route.query.fullname }}
            </AvatarFallback>
          </Avatar>
        </div>
        <div class="w-full grid grid-cols-1 p-2 py-6 gap-1 border-b">
          <div class="flex gap-2 w-full">
            <User class="text-slate-700" :size="22" />{{
              $route.query.fullname
            }}
          </div>
          <div class="flex gap-2 w-full">
            <Mail class="text-slate-700" :size="22" />{{ $route.query.email }}
          </div>
          <div class="flex gap-2 w-full">
            <Phone class="text-slate-700" :size="22" />{{
              $route.query.phoneNumber
            }}
          </div>
          <div class="flex gap-2 w-full">
            <MapPin class="text-slate-700" :size="22" />{{
              $route.query.address
            }}
          </div>
        </div>
      </div>
      <div class="h-fit w-full flex gap-2 p-2 justify-between">
        <Card class="w-full">
          <CardHeader
            class="flex flex-row items-center justify-between space-y-0 pb-2"
          >
            <CardTitle class="text-sm font-medium"> Total Revenue </CardTitle>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              class="h-4 w-4 text-muted-foreground"
            >
              <path
                d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"
              />
            </svg>
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold">{{ 0 }} DH</div>
            <p class="text-xs text-muted-foreground">
              {{ 0 }}% from last month
            </p>
          </CardContent>
        </Card>
        <Card class="w-full">
          <CardHeader
            class="flex flex-row items-center justify-between space-y-0 pb-2"
          >
            <CardTitle class="text-sm font-medium"> Total Revenue </CardTitle>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              class="h-4 w-4 text-muted-foreground"
            >
              <path
                d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"
              />
            </svg>
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold">{{ 0 }} DH</div>
            <p class="text-xs text-muted-foreground">
              {{ 0 }}% from last month
            </p>
          </CardContent>
        </Card>
      </div>
    </ScrollArea>
  </SheetContent>
</template>
