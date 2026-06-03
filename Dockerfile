# Stage 1: Build Next.js static output
FROM node:20-alpine AS frontend-build
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
ARG NEXT_PUBLIC_API_URL=/
ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL
# Mock API on :3001 so generateStaticParams can resolve slugs at build time
ENV NEXT_SERVER_API_URL=http://localhost:3001
RUN node -e "\
const http=require('http');\
const A={id:1,slug:'welcome',title:'Welcome',excerpt:'Hello',body:'Hello world',author_name:'Admin',status:'published',category_id:1,published_at:'2024-01-01T00:00:00Z',created_at:'2024-01-01T00:00:00Z',updated_at:'2024-01-01T00:00:00Z',tags:[{id:1,slug:'general',name:'General'}],category:{id:1,slug:'general',name:'General',description:''}};\
const P={items:[A],total:1,page:1,limit:20};\
const R={'/api/articles':P,'/api/articles/welcome':A,'/api/tags':[{id:1,slug:'general',name:'General'}],'/api/tags/general/articles':[A],'/api/categories':[{id:1,slug:'general',name:'General',description:''}],'/api/pages':[{id:1,slug:'about',title:'About',body:'About us',status:'published',created_at:'2024-01-01T00:00:00Z',updated_at:'2024-01-01T00:00:00Z'}],'/api/pages/about':{id:1,slug:'about',title:'About',body:'About us',status:'published',created_at:'2024-01-01T00:00:00Z',updated_at:'2024-01-01T00:00:00Z'},'/api/settings':[{key:'site_name',value:'News'},{key:'site_description',value:'A news site'},{key:'site_url',value:'http://localhost:3000'}]};\
http.createServer((req,res)=>{const p=req.url.split('?')[0];res.writeHead(200,{'Content-Type':'application/json'});res.end(JSON.stringify(R[p]||[]));}).listen(3001);\
" & sleep 1 && npm run build

# Stage 2: Build Rust binaries
FROM rust:1.88-alpine AS backend-build
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY backend/ ./
RUN cargo build --release -p server -p cli

# Stage 3: Minimal runtime image
FROM alpine:3.19
RUN apk add --no-cache ca-certificates sqlite-libs
WORKDIR /app

COPY --from=backend-build /app/target/release/server /usr/local/bin/server
COPY --from=backend-build /app/target/release/news-cli /usr/local/bin/news-cli
COPY --from=frontend-build /app/frontend/out /app/static

RUN mkdir -p /data

ENV DATABASE_URL=/data/news.db
ENV STATIC_DIR=/app/static
ENV SERVER_PORT=3000
ENV RUST_LOG=info

EXPOSE 3000
CMD ["/usr/local/bin/server"]
