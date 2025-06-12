import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarProvider,
  SidebarInset,
} from "@/components/ui/sidebar";

export default function UserScreenInitial() {
  return (
    <div className="flex flex-col h-screen bg-[#D9D9D9]">
      <div className="flex-shrink-0">
        <header className="flex items-center justify-between p-2 bg-white">
          <div className="flex items-center">
            <Button variant="ghost" className="text-white bg-[#6E30D7] hover:bg-[#5827b0] mr-2">Modelagem</Button>
            <Button variant="ghost" className="hover:bg-gray-200">Animação</Button>
            <Button variant="ghost" className="hover:bg-gray-200">Preview</Button>
            <Button variant="ghost" className="hover:bg-gray-200">AI</Button>
          </div>
          <Button className="bg-white text-black border border-black hover:bg-gray-200">Exportar</Button>
        </header>
      </div>
      <div className="flex flex-grow overflow-hidden">
        <SidebarProvider>
          <Sidebar collapsible="none" className="bg-[#171717] text-white w-64 flex flex-col">
            <SidebarHeader>
              <div className="flex border-b border-gray-600">
                <Button variant="ghost" className="flex-1 text-white border-b-2 border-white rounded-none">Histórico</Button>
                <Button variant="ghost" className="flex-1 text-gray-400 hover:text-white hover:bg-gray-700 rounded-none">Propriedades</Button>
                <Button variant="ghost" className="flex-1 text-gray-400 hover:text-white hover:bg-gray-700 rounded-none">Templates</Button>
                <Button variant="ghost" className="flex-1 text-gray-400 hover:text-white hover:bg-gray-700 rounded-none">Biblioteca</Button>
              </div>
            </SidebarHeader>
            <SidebarContent className="flex-grow">
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton className="hover:bg-gray-700 text-left justify-start">
                    Power Guido, um modelo 3d de uma empresa de p...
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton className="hover:bg-gray-700 text-left justify-start">
                    Pen is AI ain ain um modelo 3d de pe nis é bem...
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarContent>
          </Sidebar>
          <main className="flex-1 flex flex-col p-4">
            <div className="flex-grow flex items-center justify-center">
              <div className="w-64 h-64 bg-gray-500"></div>
            </div>
            <div className="flex-shrink-0">
              <Input placeholder="Prompt" className="bg-white" />
            </div>
          </main>
        </SidebarProvider>
      </div>
    </div>
  );
}
