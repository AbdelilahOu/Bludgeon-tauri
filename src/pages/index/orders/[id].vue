<script setup lang="ts">
import { onBeforeMount, ref } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";
import { error } from "tauri-plugin-log-api";
import type { Res } from "@/types";
import {
  PDFDocument,
  StandardFonts,
  rgb,
  PageSizes,
  PDFPage,
  PDFFont,
  type RGB,
} from "pdf-lib";
import fontkit from "@pdf-lib/fontkit";
import { onMounted } from "vue";
import CairoRegular from "@/assets/fonts/Cairo-Regular.ttf";

const { t, d, locale } = useI18n();
const id = useRoute().params.id;
const order = ref<any | null>(null);
const pdfRef = ref<HTMLIFrameElement | null>();
//
let resolveWaitForFetch: (value?: unknown) => void;
let waitForFetch = new Promise((r) => (resolveWaitForFetch = r));
let pdfDoc: PDFDocument;
let font: PDFFont;
let color: RGB;

onBeforeMount(async () => {
  try {
    const res = await invoke<Res<any>>("get_order_details", {
      id,
    });
    order.value = res.data;
    resolveWaitForFetch();
  } catch (err: any) {
    console.log(err);
    error("Error creating supplier : " + err.error);
  }
});

onMounted(async () => {
  try {
    await waitForFetch;
    pdfDoc = await PDFDocument.create();
    pdfDoc.registerFontkit(fontkit);
    if (locale.value == "ar-AE") {
      const res = await fetch(CairoRegular);
      const fontBytes = await res.arrayBuffer();
      font = await pdfDoc.embedFont(fontBytes);
    } else {
      font = await pdfDoc.embedFont(StandardFonts.Helvetica);
    }
    color = rgb(0.34, 0.34, 0.34);
    generatePdf();
  } catch (error) {
    console.log(error);
  }
});

const generatePdf = async () => {
  const page = pdfDoc.addPage();
  page.setSize(...PageSizes.A4);
  const { width, height } = page.getSize();

  drawOrderHeader(page, width, height, order.value);

  page.drawLine({
    start: { x: 20, y: height - 200 },
    end: { x: width - 20, y: height - 200 },
    thickness: 1,
    color,
    opacity: 0.75,
  });

  const items = [...order.value.items];
  drawOrderItems(page, width, height, items, 210);

  const pdfDataUri = await pdfDoc.saveAsBase64({ dataUri: true });
  pdfRef.value?.setAttribute("src", pdfDataUri);
};

const drawOrderHeader = (
  page: PDFPage,
  width: number,
  height: number,
  order: any,
) => {
  page.drawText("O R D E R", {
    x: width - 190,
    y: height - 40,
    font,
    size: 30,
    color,
  });
  page.drawText(order.createdAt.split(" ")[0], {
    x: width - 190,
    y: height - 70,
    font,
    size: 13,
    color,
  });
  page.drawText(t("g.status." + order.status.toLowerCase()), {
    x: width - 190,
    y: height - 90,
    font,
    size: 13,
    color,
  });
  //
  page.drawText(t("i.u.d.c.title").toUpperCase(), {
    x: 20,
    y: height - 70,
    font,
    size: 14,
    color,
  });
  page.drawText(order.supplier.fullname, {
    x: 20,
    y: height - 90,
    font,
    size: 13,
    color,
  });
  page.drawText(order.supplier.address, {
    x: 20,
    y: height - 110,
    font,
    size: 13,
    color,
  });
  page.drawText(order.supplier.phoneNumber, {
    x: 20,
    y: height - 130,
    font,
    size: 13,
    color,
  });
  page.drawText(order.supplier.email, {
    x: 20,
    y: height - 150,
    font,
    size: 13,
    color,
  });
  //
  page.drawLine({
    start: { x: 20, y: height - 170 },
    end: { x: width - 20, y: height - 170 },
    thickness: 1,
    color,
    opacity: 0.75,
  });
  page.drawText(t("g.fields.name"), {
    x: 25,
    y: height - 190,
    font,
    size: 14,
    color,
  });
  page.drawText(t("g.fields.price"), {
    x: 25 + width / 4,
    y: height - 190,
    font,
    size: 14,
    color,
  });

  page.drawText(t("g.fields.quantity"), {
    x: 25 + width / 2,
    y: height - 190,
    font,
    size: 14,
    color,
  });

  page.drawText(t("g.fields.total"), {
    x: 25 + (width * 3) / 4,
    y: height - 190,
    font,
    size: 14,
    color,
  });
};

const drawOrderItems = (
  page: PDFPage,
  width: number,
  height: number,
  items: any[],
  currentY: number,
) => {
  if (items.length === 0) {
    drawSummary(page, width, height, currentY);
    return;
  }

  const item = items.shift();
  page.drawText(item.name, {
    x: 25,
    y: height - currentY - 10,
    font,
    size: 12,
    color,
  });
  page.drawText("DH " + item.price.toFixed(2), {
    x: 25 + width / 4,
    y: height - currentY - 10,
    font,
    size: 12,
    color,
  });
  page.drawText(item.quantity.toFixed(0), {
    x: 25 + width / 2,
    y: height - currentY - 10,
    font,
    size: 12,
    color,
  });
  page.drawText("DH " + (item.price * item.quantity).toFixed(2), {
    x: 25 + (width * 3) / 4,
    y: height - currentY - 10,
    font,
    size: 12,
    color,
  });
  page.drawLine({
    start: { x: 20, y: height - currentY - 20 },
    end: { x: width - 20, y: height - currentY - 20 },
    thickness: 1,
    color,
    opacity: 0.75,
  });

  const lineHeight = 30; // Assuming a line height for each item
  const remainingHeight = height - currentY - lineHeight;
  if (remainingHeight < lineHeight + 30) {
    // Not enough space, create a new page and continue
    const newPage = pdfDoc.addPage();
    drawOrderItems(newPage, width, height, items, lineHeight);
  } else {
    // Enough space, continue drawing on current page
    drawOrderItems(page, width, height, items, currentY + lineHeight);
  }
};

const drawSummary = (
  page: PDFPage,
  width: number,
  height: number,
  currentY: number,
) => {
  page.drawText("DH " + order.value.total.toFixed(2), {
    x: 25 + (width * 3) / 4,
    y: height - currentY - 10,
    font,
    size: 12,
    color,
  });

  page.drawText(t("g.fields.total"), {
    x: 100 + (width * 2) / 4,
    y: height - currentY - 10,
    font,
    size: 12,
    color,
  });
  page.drawLine({
    start: { x: 90 + (width * 2) / 4, y: height - currentY - 20 },
    end: { x: width - 20, y: height - currentY - 20 },
    thickness: 1,
    color,
    opacity: 0.75,
  });
};
</script>

<template>
  <main class="w-full h-screen">
    <iframe ref="pdfRef" style="width: 100%; height: 100%" />
  </main>
</template>
