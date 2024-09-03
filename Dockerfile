FROM debian:buster

# 换国内源
RUN echo "deb http://mirrors.aliyun.com/debian/ buster main contrib non-free" > /etc/apt/sources.list && \
    echo "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ buster main contrib non-free" >> /etc/apt/sources.list && \
    echo "deb http://mirrors.163.com/debian/ buster main contrib non-free" >> /etc/apt/sources.list && \
    echo "deb http://mirrors.ustc.edu.cn/debian/ buster main contrib non-free" >> /etc/apt/sources.list

# 更新软件包列表和安装基本工具
RUN apt-get update && apt-get install -y ca-certificates curl gnupg2 software-properties-common apt-transport-https vim

# 更新包列表并安装必要的软件
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    # 安装 Zsh
    && apt-get install -y zsh \
    && chsh -s $(which zsh) root \
    && rm -rf /var/lib/apt/lists/*


# 将 Rust 的 Cargo 二进制文件添加到 PATH 环境变量
RUN echo 'source $HOME/.cargo/env' >> $HOME/.zshrc

# 安装 zsh
RUN apt-get install -y zsh

# 将zsh替换为你的默认shell
RUN chsh -s /bin/zsh


# 设置工作目录
WORKDIR /codes

# 添加 FNM 到 PATH
# RUN source ~/.nvm/nvm.sh

# 将宿主机的代码目录挂载到容器中
VOLUME ["/codes"]

# 暴露端口
EXPOSE 7000

# 启动命令
CMD ["zsh"]

################
# 宿主机命令行  #
################

# 构建Docker镜像：在d:/apps目录下打开命令行工具，运行以下命令来构建镜像：
# docker build -t rs-env .

# 运行Docker容器
# docker run -it --name rsd -v /d/apps:/codes -p 8080:8080 fs-env

# 运行 Docker容器，wsl目录
# docker run -it --name rsd -v \\wsl$\ubuntu\home\kv\apps:/codes -p 8080:8080 rs-env